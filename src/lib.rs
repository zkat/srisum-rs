mod check;
mod compute;
mod errors;
use std::ffi::OsString;

pub use check::check;
pub use compute::compute;

use clap::Parser;

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct CliArgs {
    /// Files to process, depending on mode.
    #[clap(name = "FILE", default_value = "-")]
    pub files: Vec<OsString>,

    /// Hash algorithms to generate for the FILEs.
    #[clap(short, long, default_value = "sha256")]
    pub algorithms: Vec<String>,

    /// Print digests only, without the filename.
    #[clap(short, long)]
    pub digest_only: bool,

    /// Read integrity checksums from the FILEs and check them.
    #[clap(short, long)]
    pub check: bool,

    /// Don't fail or report status for missing files.
    #[clap(long)]
    pub ignore_missing: bool,

    /// Don't print OK for each successfully verified file.
    #[clap(short, long)]
    pub quiet: bool,

    /// Don't output anything. Status code shows success.
    #[clap(long)]
    pub status: bool,

    /// Warn about improperly formatted checksum lines.
    #[clap(short, long)]
    warn: bool
}
