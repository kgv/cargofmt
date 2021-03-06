use anyhow::Result;
use cargo_pretty::{Format, Settings};
use std::fs::write;
use toml_lalrpop::TomlParser;

fn main() -> Result<()> {
    const MANIFEST: &str = include_str!("./source/Cargo.toml");
    let mut manifest = TomlParser::new().parse(MANIFEST)?;
    let settings = Settings::default();
    let formated = format!("{}", manifest.format(&settings));
    write("./examples/serde/target/Cargo.toml", formated)?;
    Ok(())
}
