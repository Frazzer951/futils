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

        #[arg(short, long, help = "Minimum length for the comment.")]
        min_length: Option<usize>,

        #[arg(short, long, help = "Symbol to use for the comment.")]
        symbol: Option<char>,

        #[arg(short, long, help = "Convert the comment to uppercase.")]
        caps: bool,

        #[arg(short, long, help = "Prefix to use for the comment.")]
        prefix: Option<String>,
    },
    FormatJson {
        filename: String,

        #[arg(short, long, help = "Optional output filename to write the formatted JSON.")]
        output: Option<String>,

        #[arg(short, long, help = "Sort the JSON keys.")]
        sort: bool,
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
        Some(Commands::FormatJson { filename, output, sort }) => {
            format_json_file(filename, output.as_deref(), *sort)?;
        },
        None => {
            bail!("Unknown command")
        },
    }

    Ok(())
}
