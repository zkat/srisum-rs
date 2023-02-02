use std::ffi::OsStr;
use std::fs::File;
use std::io::{self, prelude::*, BufReader, Read};

use miette::{IntoDiagnostic, Report, Result, WrapErr};
use ssri::{Algorithm, Integrity, IntegrityChecker};

use crate::errors::SrisumError;
use crate::CliArgs;

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

pub fn check(args: CliArgs) -> Result<()> {
    let files = &args.files;
    let mut stats = Stats::new();
    let marker = OsStr::new("-");
    for f in files {
        let handle: Box<dyn Read> = if marker == f {
            Box::new(std::io::stdin())
        } else {
            Box::new(File::open(&f).into_diagnostic().wrap_err_with(|| {
                format!("Failed to open checksum file: {}", f.to_string_lossy())
            })?)
        };
        handle_stream(&mut stats, BufReader::new(handle), &args).context("CRITICAL ERROR")?;
    }
    print_warnings(&stats, &args);
    if stats.bad_lines > 0 || stats.bad_checksums > 0 || stats.missing_files > 0 {
        Err(miette::miette!("Checksum failed"))
    } else {
        Ok(())
    }
}

fn handle_stream<T: Read>(stats: &mut Stats, s: BufReader<T>, args: &CliArgs) -> Result<()> {
    // TODO - use s.split() to support OsStr filenames in the RHS.
    for line in s.lines() {
        // Lines must unfortunately be valid UTF-8 right now (a restriction
        // that only applies to check, but not to compute). This will be the
        // case until such a time when it's deemed necessary to go through the
        // pain of manually reading out these lines in their OsStr form. Fuck
        // that, honestly. So, we treat bad UTF-8 lines (or any other encoding error) as bad lines and just keep going.
        let line = if let Ok(line) = line {
            line
        } else {
            stats.bad_lines += 1;
            continue;
        };
        let line = line.trim();
        // Empty lines are fine, just continue.
        if line.is_empty() {
            continue;
        }
        // Split each line into `<hash>\s+<filename>`
        let split = String::from(line).chars().position(|x| x.is_whitespace());
        // No \s+ in the middle means we got a bad line. Just skip it.
        let idx = if let Some(split) = split {
            split
        } else {
            stats.bad_lines += 1;
            continue;
        };
        let (hash, filename) = line.split_at(idx);
        let filename = filename.trim();
        let sri = hash.parse::<Integrity>();
        // If the integrity string fails to parse... that's a bad line. Skip.
        let sri = if let Ok(sri) = sri {
            sri
        } else {
            stats.bad_lines += 1;
            continue;
        };
        let result = check_file(filename, sri);
        match result {
            Ok((algo, f)) => {
                if !args.status && !args.quiet {
                    println!("{}: OK ({})", f, algo)
                }
            }
            Err(cause) => {
                print_messages(cause, stats, args, filename)?;
            }
        };
    }
    Ok(())
}

fn print_messages(cause: Report, stats: &mut Stats, args: &CliArgs, filename: &str) -> Result<()> {
    match cause.downcast::<SrisumError>() {
        // ENOENT
        Ok(SrisumError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => {
            stats.missing_files += 1;
            if !args.ignore_missing {
                let current_exe = std::env::current_exe().into_diagnostic()?;
                let current_exe_display = current_exe.display();
                eprintln!("{current_exe_display}: {filename}: No such file or directory");
                println!("{filename}: FAILED open or read");
            }
        }
        // EINTEGRITY
        Ok(SrisumError::IntegrityError(_)) => {
            stats.bad_checksums += 1;
            if !args.status && !args.quiet {
                println!("{}: FAILED", filename);
            }
        }
        Ok(err) => {
            return Err(err.into());
        }
        // Other errors
        Err(err) => {
            return Err(err);
        }
    }
    Ok(())
}

fn check_file(f: &str, sri: Integrity) -> Result<(Algorithm, String)> {
    let stream: Box<dyn Read> = if f == "-" {
        Box::new(std::io::stdin())
    } else {
        Box::new(File::open(&f).into_diagnostic()?)
    };
    let mut reader = BufReader::new(stream);
    let mut checker = IntegrityChecker::new(sri);
    let mut buf = [0; 1024 * 256];
    loop {
        let amt = reader.read(&mut buf).into_diagnostic()?;
        if amt == 0 {
            break;
        } else {
            checker.input(&buf[0..amt]);
        }
    }
    let algo = checker.result().map_err(SrisumError::IntegrityError)?;
    Ok((algo, String::from(f)))
}

fn print_warnings(stats: &Stats, args: &CliArgs) {
    if args.status {
        return;
    }
    if args.warn && stats.bad_lines > 0 {
        eprintln!(
            "srisum: WARNING: {} file(s) could not be read",
            stats.bad_lines
        );
    }
    if !args.ignore_missing && stats.missing_files > 0 {
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
