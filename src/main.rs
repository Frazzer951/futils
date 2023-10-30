mod comment;
mod format_json;

use anyhow::{bail, Result};
use clap::{Parser, Subcommand};
use comment::comment;
use format_json::format_json_file;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Comment {
        text: String,

        #[arg(short, long)]
        min_length: Option<usize>,

        #[arg(short, long)]
        symbol: Option<char>,

        #[arg(short, long)]
        caps: bool,

        #[arg(short, long)]
        prefix: Option<String>,
    },
    FormatJson {
        filename: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Comment {
            text,
            min_length,
            symbol,
            caps,
            prefix,
        }) => {
            let text = if *caps { text.to_uppercase() } else { text.to_string() };
            println!("{}", comment(&text, *min_length, *symbol, prefix.clone()))
        },
        Some(Commands::FormatJson { filename }) => {
            format_json_file(filename)?;
        },
        None => {
            bail!("Unknown command")
        },
    }

    Ok(())
}
