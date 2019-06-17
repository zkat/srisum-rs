mod check;
mod compute;
pub use check::check;
pub use compute::compute;

use clap::{Arg, App, crate_version};

pub fn parse_args<'a, 'b>() -> App<'a, 'b> {
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

