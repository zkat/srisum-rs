use anyhow::Result;
use srisum;

fn main() -> Result<()> {
    let matches = srisum::parse_args().get_matches();
    if matches.is_present("check") {
        srisum::check(matches)?;
    } else {
        srisum::compute(matches)?;
    }
    Ok(())
}
