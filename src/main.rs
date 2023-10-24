mod comment;

use clap::{Parser, Subcommand};
use comment::comment;

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
}

fn main() {
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
        None => {
            panic!("Unknown command")
        },
    }
}
