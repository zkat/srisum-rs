use std::ffi::OsStr;
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;
use std::fs::File;
use std::io::{BufReader, Read, Write};

use clap::{Arg, App, crate_version, ArgMatches};
use ssri::{Builder, Integrity, Algorithm};

fn main() {
    let matches = parse_args().get_matches();
    if matches.is_present("check") {
        check(matches)
    } else {
        compute(matches)
    }
}

fn parse_args<'a, 'b>() -> App<'a, 'b> {
    App::new("srisum")
        .version(crate_version!())
        .author("Kat Marchán <kzm@zkat.tech>")
        .about("Compute and check subresource integrity digests.")
        .arg(Arg::with_name("FILE")
            .multiple(true)
            .default_value("-")
            .index(1)
            .help("files to process, depending on mode"))
        .arg(Arg::with_name("algorithms")
            .short("a")
            .long("algorithms")
            .value_name("ALGO")
            .takes_value(true)
            .multiple(true)
            .default_value("sha256")
            .help("hash algorithms to generate for the FILEs"))
        .arg(Arg::with_name("digest-only")
            .short("d")
            .long("digest-only")
            .help("print digests only, without the filename"))
        .arg(Arg::with_name("check")
            .short("c")
            .long("check")
            .help("read integrity checksums from the FILEs and check them"))
        .arg(Arg::with_name("ignore-missing")
            .long("ignore-missing")
            .help("don't fail or report status for missing files"))
        .arg(Arg::with_name("quiet")
            .short("q")
            .long("quiet")
            .help("don't print OK for each successfully verified file"))
        .arg(Arg::with_name("status")
            .long("status")
            .help("don't output anything, status code shows success"))
        .arg(Arg::with_name("warn")
            .short("w")
            .long("warn")
            .help("warn about improperly formatted checksum lines"))
}

fn check(matches: ArgMatches) {

}

fn compute(matches: ArgMatches) {
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
    let mut buf = [0; 1024 * 1024];
    loop {
        let amt = reader.read(&mut buf)?;
        if amt == 0 {
            return Ok(());
        } else {
            builder.input(&buf[0..amt]);
        }
    }
}
