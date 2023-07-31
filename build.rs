use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("Entrypoint", "abis/EntryPoint.json")?
        .generate()?
        .write_to_file("src/abi/entry_point.rs")?;

    Ok(())
}