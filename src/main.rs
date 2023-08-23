use anyhow::Result;
use fch::dictionary::parse_inputs;

fn main() -> Result<()> {
    let mut dictionary = parse_inputs()?;
    let mut i = true;
    while i {
        (i) = dictionary.layer_lookup();
    }
    Ok(())
}
