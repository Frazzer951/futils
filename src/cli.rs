use crate::{comment::comment, format_json::format_json_file};
use anyhow::{bail, Result};
use clap::{command, value_parser, Arg, ArgAction, Command};

fn cli() -> Command {
    command!()
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![subcommand_comment(), subcommand_format_json()])
}

fn subcommand_comment() -> Command {
    Command::new("comment").about("Create a comment").args(&[
        Arg::new("text").required(true).index(1).help("Text for the comment"),
        Arg::new("min_length")
            .short('m')
            .long("min-length")
            .help("Minimum length for the comment."),
        Arg::new("symbol")
            .short('s')
            .long("symbol")
            .help("Symbol to use for the comment.")
            .value_parser(value_parser!(char)),
        Arg::new("prefix")
            .short('p')
            .long("prefix")
            .help("Prefix to use for the comment."),
        Arg::new("caps")
            .short('c')
            .long("caps")
            .help("Convert the comment to uppercase.")
            .action(ArgAction::SetTrue),
    ])
}

fn subcommand_format_json() -> Command {
    Command::new("format-json").about("Format a JSON file").args(&[
        Arg::new("filename")
            .required(true)
            .index(1)
            .help("Filename of the JSON to format"),
        Arg::new("output")
            .short('o')
            .long("output")
            .help("Optional output filename to write the formatted JSON."),
        Arg::new("sort")
            .short('s')
            .long("sort")
            .help("Sort the JSON keys.")
            .action(ArgAction::SetTrue),
    ])
}

pub fn parse() -> Result<()> {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("comment", sub_matches)) => {
            let text = sub_matches.get_one::<String>("text").unwrap().clone();
            let min_length = sub_matches.get_one::<usize>("min_length").cloned();
            let symbol = sub_matches.get_one::<char>("symbol").cloned();
            let prefix = sub_matches.get_one::<String>("prefix").cloned();
            let caps = sub_matches.get_flag("caps");

            let text = if caps { text.to_uppercase() } else { text };
            println!("{}", comment(&text, min_length, symbol, prefix))
        },
        Some(("format-json", sub_matches)) => {
            let filename = sub_matches.get_one::<String>("filename").unwrap().clone();
            let output = sub_matches.get_one::<String>("output").cloned();
            let sort = sub_matches.get_flag("sort");

            format_json_file(filename, output, sort)?;
        },
        Some((command, _)) => {
            bail!("Code has not yet been written for `{command}`");
        },
        _ => unreachable!(),
    }

    Ok(())
}
