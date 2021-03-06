use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read};

use anyhow::{anyhow, Context, Result};
use clap::ArgMatches;
use ssri::{Algorithm, Integrity, IntegrityChecker};

use crate::errors::SrisumError;

struct Stats {
    bad_lines: u64,
    bad_checksums: u64,
    missing_files: u64,
}

impl Stats {
    fn new() -> Stats {
        Stats {
            bad_lines: 0,
            bad_checksums: 0,
            missing_files: 0,
        }
    }
}

pub fn check(matches: ArgMatches) -> Result<()> {
    let files = matches
        .values_of_os("FILE")
        .context("Something went wrong reading your files")?;
    let mut stats = Stats::new();
    let marker = OsStr::new("-");
    for f in files.into_iter() {
        let handle: Box<dyn Read> = if marker == f {
            Box::new(std::io::stdin())
        } else {
            Box::new(File::open(&f).with_context(|| {
                format!("Failed to open checksum file: {}", f.to_string_lossy())
            })?)
        };
        handle_stream(&mut stats, BufReader::new(handle), &matches).context("CRITICAL ERROR")?;
    }
    print_warnings(&stats, matches);
    if stats.bad_lines > 0 || stats.bad_checksums > 0 || stats.missing_files > 0 {
        Err(anyhow!("Checksum failed"))
    } else {
        Ok(())
    }
}

fn handle_stream<T: Read>(stats: &mut Stats, s: BufReader<T>, matches: &ArgMatches) -> Result<()> {
    // TODO - use s.split() to support OsStr filenames in the RHS.
    for line in s.lines() {
        // Lines must unfortunately be valid UTF-8 right now (a restriction
        // that only applies to check, but not to compute). This will be the
        // case until such a time when it's deemed necessary to go through the
        // pain of manually reading out these lines in their OsStr form. Fuck
        // that, honestly. So, we treat bad UTF-8 lines (or any other encoding error) as bad lines and just keep going.
        let line = if line.is_err() {
            stats.bad_lines += 1;
            continue;
        } else {
            line.unwrap()
        };
        let line = line.trim();
        // Empty lines are fine, just continue.
        if line.is_empty() {
            continue;
        }
        // Split each line into `<hash>\s+<filename>`
        let split = String::from(line).chars().position(|x| x.is_whitespace());
        // No \s+ in the middle means we got a bad line. Just skip it.
        let idx = if split.is_none() {
            stats.bad_lines += 1;
            continue;
        } else {
            split.unwrap()
        };
        let (hash, filename) = line.split_at(idx);
        let filename = filename.trim();
        let sri = hash.parse::<Integrity>();
        // If the integrity string fails to parse... that's a bad line. Skip.
        let sri = if sri.is_err() {
            stats.bad_lines += 1;
            continue;
        } else {
            sri.unwrap()
        };
        let result = check_file(filename, sri);
        match result {
            Ok((algo, f)) => {
                if !matches.is_present("status") && !matches.is_present("quiet") {
                    println!("{}: OK ({})", f, algo)
                }
            }
            Err(cause) => {
                print_messages(cause, stats, &matches, filename)?;
            }
        };
    }
    Ok(())
}

fn print_messages(
    cause: anyhow::Error,
    stats: &mut Stats,
    matches: &ArgMatches,
    filename: &str,
) -> Result<()> {
    match cause.downcast::<SrisumError>() {
        // ENOENT
        Ok(SrisumError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            stats.missing_files += 1;
            if !matches.is_present("ignore-missing") {
                // TODO - Use argv[0]
                eprintln!("srisum: {}: No such file or directory", filename);
                println!("{}: FAILED open or read", filename);
            }
        }
        // EINTEGRITY
        Ok(SrisumError::IntegrityError(_)) => {
            stats.bad_checksums += 1;
            if !matches.is_present("status") && !matches.is_present("quiet") {
                println!("{}: FAILED", filename);
            }
        }
        Ok(err) => {
            Err(err)?;
        }
        // Other errors
        Err(err) => {
            Err(err)?;
        }
    }
    Ok(())
}

fn check_file(f: &str, sri: Integrity) -> Result<(Algorithm, String)> {
    let stream: Box<dyn Read> = if f == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(&f)?)
    };
    let mut reader = BufReader::new(stream);
    let mut checker = IntegrityChecker::new(sri);
    let mut buf = [0; 1024 * 256];
    loop {
        let amt = reader.read(&mut buf)?;
        if amt == 0 {
            break;
        } else {
            checker.input(&buf[0..amt]);
        }
    }
    let algo = checker
        .result()
        .map_err(|err| SrisumError::IntegrityError(err))?;
    Ok((algo, String::from(f)))
}

fn print_warnings(stats: &Stats, matches: ArgMatches) {
    if matches.is_present("status") {
        return;
    }
    if matches.is_present("warn") && stats.bad_lines > 0 {
        eprintln!(
            "srisum: WARNING: {} file(s) could not be read",
            stats.bad_lines
        );
    }
    if !matches.is_present("ignore-missing") && stats.missing_files > 0 {
        eprintln!(
            "srisum: WARNING: {} listed file(s) could not be read",
            stats.missing_files
        );
    }
    if stats.bad_checksums > 0 {
        eprintln!(
            "srisum: WARNING: {} computed checksum(s) did NOT match",
            stats.bad_checksums
        )
    }
}
