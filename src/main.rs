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
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Comment {
            text,
            min_length,
            symbol,
        }) => {
            println!("{}", comment(text, *min_length, *symbol))
        },
        None => {
            panic!("Unknown command")
        },
    }
}
