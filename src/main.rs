mod cli;
mod comment;
mod format_json;
mod format_sql;

use anyhow::Result;

fn main() -> Result<()> {
    cli::parse()?;

    Ok(())
}
