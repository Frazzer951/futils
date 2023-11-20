use crate::{
    comment::comment,
    format_json::{format_json_file, FormatJsonConfig},
};
use anyhow::{bail, Result};
use clap::{command, value_parser, Arg, ArgAction, Command, ValueHint};
use clap_complete::{generate, Generator, Shell};
use std::io;

fn cli() -> Command {
    command!("futils")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(vec![subcommand_comment(), subcommand_format_json(), subcommand_generator()])
}

fn subcommand_comment() -> Command {
    Command::new("comment").about("Create a comment").args(&[
        Arg::new("text").required(true).help("Text for the comment"),
        Arg::new("min_length")
            .short('m')
            .long("min-length")
            .help("Minimum length for the comment.")
            .value_parser(value_parser!(usize)),
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
            .help("Filename of the JSON to format")
            .value_hint(ValueHint::AnyPath),
        Arg::new("indent")
            .short('t')
            .long("indent")
            .help("Indent size to use for the formatted JSON.")
            .value_parser(value_parser!(usize)),
        Arg::new("output")
            .short('o')
            .long("output")
            .help("Optional output filename to write the formatted JSON.")
            .value_hint(ValueHint::AnyPath),
        Arg::new("sort")
            .short('s')
            .long("sort")
            .help("Sort the JSON keys.")
            .action(ArgAction::SetTrue),
        Arg::new("in-place")
            .short('i')
            .long("in-place")
            .help("Format the file in-place.")
            .action(ArgAction::SetTrue),
    ])
}

fn subcommand_generator() -> Command {
    Command::new("generate")
        .about("Generate Shell Completions")
        .after_help("Example Usage: futils generate zsh > ~/.completions/_futils")
        .args(&[Arg::new("shell")
            .required(true)
            .help("The shell to generate completions for.")
            .action(ArgAction::Set)
            .value_parser(value_parser!(Shell))])
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
            let indent = sub_matches.get_one::<usize>("indent").cloned();
            let output = sub_matches.get_one::<String>("output").cloned();
            let sort = sub_matches.get_flag("sort");
            let in_place = sub_matches.get_flag("in-place");

            let should_print = output.is_none() && !in_place;

            let json_config = FormatJsonConfig {
                filename,
                output_filename: output,
                indent_size: indent,
                sort,
                in_place,
            };

            let formatted_json = format_json_file(json_config)?;
            if should_print {
                println!("{}", formatted_json);
            }
        },
        Some(("generate", sub_matches)) => {
            let generator = sub_matches.get_one::<Shell>("shell").unwrap();
            let mut cmd = cli();
            eprintln!("Generating completion file for {generator}...");
            print_completions(*generator, &mut cmd);
        },
        Some((command, _)) => {
            bail!("Code has not yet been written for `{command}`");
        },
        _ => unreachable!(),
    }

    Ok(())
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut io::stdout());
}
