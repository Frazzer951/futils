mod cli;
mod comment;
mod format_json;

use anyhow::Result;

fn main() -> Result<()> {
    cli::parse()?;

    Ok(())
}
