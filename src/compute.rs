use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use clap::ArgMatches;
use ssri::{Builder, Integrity, Algorithm};

pub fn compute(matches: ArgMatches) {
    let files = matches
        .values_of_os("FILE")
        .expect("Something went wrong reading your files");
    let mut exit = 0;
    for f in files.into_iter() {
        match hash_file(f, &matches) {
            Ok(sri) => {
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
                    let output: Vec<u8> = f.encode_wide().collect();

                    std::io::stdout().write_all(&output[..])
                        .expect("failed to write out filename");
                    println!();
                }
            }
            Err(err) => {
                exit = 1;
                eprintln!("{}", err);
            }
        }
    }
    ::std::process::exit(exit);
}

fn hash_file(f: &OsStr, matches: &ArgMatches) -> Result<Integrity, std::io::Error> {
    let mut builder = Builder::new();
    for algo in matches.values_of("algorithms").unwrap().into_iter() {
        let algo = match algo {
            "sha1" => Algorithm::Sha1,
            "sha256" => Algorithm::Sha256,
            "sha384" => Algorithm::Sha384,
            "sha512" => Algorithm::Sha512,
            _ => panic!("bad algorithm: {}", algo)
        };
        builder = builder.algorithm(algo);
    };

    if f == OsStr::new("-") {
        read_from_file(BufReader::new(std::io::stdin()), &mut builder)?;
    } else {
        read_from_file(BufReader::new(File::open(&f)?), &mut builder)?;
    };
    Ok(builder.result())
}

fn read_from_file<T: Read>(mut reader: BufReader<T>, builder: &mut Builder) -> Result<(), std::io::Error> {
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
