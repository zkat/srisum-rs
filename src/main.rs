use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;

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
                    println!("{} {}", sri, f.to_string_lossy())
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
    let mut buf = String::new();
    if f == OsStr::new("-") {
        std::io::stdin().read_to_string(&mut buf)?;
    } else {
        File::open(&f)?.read_to_string(&mut buf)?;
    };
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
    Ok(builder.chain(buf).result())
}
