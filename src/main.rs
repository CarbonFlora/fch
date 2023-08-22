use anyhow::Result;
use fch::dictionary::{layer_lookup, parse_inputs};

fn main() -> Result<()> {
    let mut dictionary = parse_inputs()?;
    let mut i = true;
    while i {
        (i) = layer_lookup(&mut dictionary);
    }
    Ok(())
}
