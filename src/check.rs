use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read};

use clap::ArgMatches;
use ssri::{Algorithm, Integrity, IntegrityChecker};

use crate::errors::Error;

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

pub fn check(matches: ArgMatches) {
    let files = matches
        .values_of_os("FILE")
        .expect("Something went wrong reading your files");
    let mut stats = Stats::new();
    let marker = OsStr::new("-");
    for f in files.into_iter() {
        let handle: Box<dyn Read> = if marker == f {
            Box::new(std::io::stdin())
        } else {
            Box::new(File::open(&f).unwrap_or_else(|_| {
                eprintln!(
                    "srisum: failed to open checksum file: {}",
                    f.to_string_lossy()
                );
                std::process::exit(1);
            }))
        };
        match handle_stream(&mut stats, BufReader::new(handle), &matches) {
            Ok(()) => {}
            Err(err) => {
                eprintln!("srisum: CRITICAL ERROR: {}", err);
                std::process::exit(1);
            }
        }
    }
    print_warnings(&stats, matches);
    if stats.bad_lines > 0 || stats.bad_checksums > 0 || stats.missing_files > 0 {
        std::process::exit(1);
    }
}

fn handle_stream<T: Read>(
    stats: &mut Stats,
    s: BufReader<T>,
    matches: &ArgMatches,
) -> Result<(), Error> {
    // TODO - use s.split() to support OsStr filenames in the RHS.
    for line in s.lines() {
        // Unwrap the line -- it must be valid UTF8
        if line.is_err() {
            stats.bad_lines += 1;
            continue;
        };
        let line = line.unwrap();
        let line = line.trim();
        // Empty lines are fine, just continue.
        if line.is_empty() {
            continue;
        }
        // Split each line into `<hash>\s+<filename>`
        let split = String::from(line).chars().position(|x| x.is_whitespace());
        // No \s+ in the middle means we got a bad line. Just skip it.
        if split.is_none() {
            stats.bad_lines += 1;
            continue;
        }
        let idx = split.unwrap(); // Never panics.
        let (hash, filename) = line.split_at(idx);
        let filename = filename.trim();
        let sri = hash.parse::<Integrity>();
        // If the integrity string fails to parse... that's a bad line. Skip.
        if sri.is_err() {
            stats.bad_lines += 1;
            continue;
        }
        let sri = sri.unwrap(); // Never panics.
        let result = check_file(filename, sri);
        match result {
            Ok((algo, f)) => {
                if !matches.is_present("status") && !matches.is_present("quiet") {
                    println!("{}: OK ({})", f, algo)
                }
            }
            // ENOENT
            Err(Error::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
                stats.missing_files += 1;
                if !matches.is_present("ignore-missing") {
                    // TODO - Use argv[0]
                    eprintln!("srisum: {}: No such file or directory", filename);
                    println!("{}: FAILED open or read", filename);
                }
            }
            // EINTEGRITY
            Err(Error::IntegrityError(f)) => {
                stats.bad_checksums += 1;
                if !matches.is_present("status") && !matches.is_present("quiet") {
                    println!("{}: FAILED", f);
                }
            }
            // Other errors
            Err(err) => Err(err)?,
        };
    }
    Ok(())
}

fn check_file(f: &str, sri: Integrity) -> Result<(Algorithm, String), Error> {
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
    if let Ok(algo) = checker.result() {
        Ok((algo, String::from(f)))
    } else {
        Err(Error::IntegrityError(String::from(f)))
    }
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
