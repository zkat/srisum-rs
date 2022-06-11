use std::ffi::OsStr;
use std::fs::File;
use std::io::{BufReader, Read, Write};
#[cfg(unix)]
use std::os::unix::ffi::OsStrExt;
#[cfg(windows)]
use std::os::windows::ffi::OsStrExt;

use miette::{IntoDiagnostic, Result, WrapErr};
use ssri::{Algorithm, Integrity, IntegrityOpts};

use crate::CliArgs;

pub fn compute(args: CliArgs) -> Result<()> {
    let files = &args.files;
    for f in files {
        let sri = hash_file(f, &args)
            .with_context(|| format!("Failed to hash file: {}", f.to_string_lossy()))?;
        if args.digest_only {
            println!("{}", sri);
        } else {
            print!("{} ", sri);

            #[cfg(unix)]
            let output = f.as_bytes();

            #[cfg(windows)]
            let output: Vec<u16> = f.encode_wide().collect();

            let stdout = std::io::stdout();
            let mut lock = stdout.lock();
            for item in output {
                lock.write_all(&item.to_be_bytes())
                    .into_diagnostic()
                    .wrap_err("Failed to write filename to stdout.")?;
            }
            println!();
        }
    }
    Ok(())
}

fn hash_file(f: &OsStr, args: &CliArgs) -> Result<Integrity> {
    let mut builder = IntegrityOpts::new();
    for algo in &args.algorithms {
        let algo = match algo.as_str() {
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
        read_from_file(
            BufReader::new(File::open(&f).into_diagnostic()?),
            &mut builder,
        )?;
    };
    Ok(builder.result())
}

fn read_from_file<T: Read>(mut reader: BufReader<T>, builder: &mut IntegrityOpts) -> Result<()> {
    let mut buf = [0; 1024 * 256];
    loop {
        let amt = reader.read(&mut buf).into_diagnostic()?;
        if amt == 0 {
            return Ok(());
        } else {
            builder.input(&buf[0..amt]);
        }
    }
}
