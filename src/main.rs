use std::env;
use std::fs;
use std::io::{self, Read};
use std::process::ExitCode;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.first().map(String::as_str) {
        Some("tsv2md") => run_tsv2md(&args[1..]),
        Some("csv2md") => run_csv2md(&args[1..]),
        Some("-h") | Some("--help") | None => {
            print_help();
            ExitCode::SUCCESS
        }
        Some(other) => {
            eprintln!("error: unknown subcommand '{other}'");
            print_help();
            ExitCode::FAILURE
        }
    }
}

fn run_tsv2md(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("Usage: biox tsv2md [FILE]\n\nConvert TSV from FILE or stdin into a Markdown table.");
        return ExitCode::SUCCESS;
    }

    let input = match args.first() {
        Some(path) => match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("error: failed to read '{path}': {err}");
                return ExitCode::FAILURE;
            }
        },
        None => {
            let mut buffer = String::new();
            if let Err(err) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("error: failed to read stdin: {err}");
                return ExitCode::FAILURE;
            }
            buffer
        }
    };

    print!("{}", biox_tsv2md::tsv2md(&input));
    ExitCode::SUCCESS
}

fn run_csv2md(args: &[String]) -> ExitCode {
    if args.iter().any(|a| a == "-h" || a == "--help") {
        println!("Usage: biox csv2md [FILE]\n\nConvert CSV from FILE or stdin into a Markdown table.");
        return ExitCode::SUCCESS;
    }

    let input = match args.first() {
        Some(path) => match fs::read_to_string(path) {
            Ok(contents) => contents,
            Err(err) => {
                eprintln!("error: failed to read '{path}': {err}");
                return ExitCode::FAILURE;
            }
        },
        None => {
            let mut buffer = String::new();
            if let Err(err) = io::stdin().read_to_string(&mut buffer) {
                eprintln!("error: failed to read stdin: {err}");
                return ExitCode::FAILURE;
            }
            buffer
        }
    };

    print!("{}", biox_tsv2md::csv2md(&input));
    ExitCode::SUCCESS
}

fn print_help() {
    println!("biox - a bioinformatics toolbox\n\nUsage: biox <SUBCOMMAND>\n\nSubcommands:\n  tsv2md [FILE]    Convert TSV to a Markdown table\n  csv2md [FILE]    Convert CSV to a Markdown table");
}
