use clap::Parser;
use miette::Result;
use srisum::CliArgs;

fn main() -> Result<()> {
    let args = CliArgs::parse();
    if args.check {
        srisum::check(args)?;
    } else {
        srisum::compute(args)?;
    }
    Ok(())
}
