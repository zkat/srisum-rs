use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read, Write};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

use anyhow::{Context, Result};
use clap::ArgMatches;
use ssri::{Algorithm, Integrity, IntegrityOpts};

pub fn compute(matches: ArgMatches) -> Result<()> {
    let files = matches
        .values_of_os("FILE")
        .context("Something went wrong reading your files.")?;
    for f in files.into_iter() {
        let sri = hash_file(f, &matches)
            .with_context(|| format!("Failed to hash file: {}", f.to_string_lossy()))?;
        if matches.is_present("digest-only") {
            println!("{}", sri);
        } else {
            // NOTE: This whole dance is to allow us to print out
            // the OsStr for the filenames exactly as we received
            // them. It's a little bit of overkill, but it guarantees
            // the user gets back what they gave and bypasses any
            // weird encoding issues with these filenames.
            print!("{} ", sri);

            #[cfg(unix)]
            let output = f.as_bytes();

            #[cfg(windows)]
            let output: Vec<u16> = f.encode_wide().collect();

            let stdout = std::io::stdout();
            let mut lock = stdout.lock();
            for item in output {
                lock.write_all(&item.to_be_bytes())
                    .context("Failed to write filename to stdout.")?;
            }
            println!();
        }
    }
    Ok(())
}

fn hash_file(f: &OsStr, matches: &ArgMatches) -> Result<Integrity> {
    let mut builder = IntegrityOpts::new();
    for algo in matches.values_of("algorithms").unwrap().into_iter() {
        let algo = match algo {
            "sha1" => Algorithm::Sha1,
            "sha256" => Algorithm::Sha256,
            "sha384" => Algorithm::Sha384,
            "sha512" => Algorithm::Sha512,
            _ => panic!("bad algorithm: {}", algo),
        };
        builder = builder.algorithm(algo);
    }

    if f == OsStr::new("-") {
        read_from_file(BufReader::new(std::io::stdin()), &mut builder)?;
    } else {
        read_from_file(BufReader::new(File::open(&f)?), &mut builder)?;
    };
    Ok(builder.result())
}

fn read_from_file<T: Read>(mut reader: BufReader<T>, builder: &mut IntegrityOpts) -> Result<()> {
    let mut buf = [0; 1024 * 256];
    loop {
        let amt = reader.read(&mut buf)?;
        if amt == 0 {
            return Ok(());
        } else {
            builder.input(&buf[0..amt]);
        }
    }
}
