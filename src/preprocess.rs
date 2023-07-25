// use crate::errors::{Error, ErrorKind};
use clap::Parser;
use colored::Colorize;
use std::path::Path;

#[derive(Debug, Parser, Clone)]
#[clap(about, version, author)]
pub struct Args {
    /// Path to the file to interpret
    #[clap(value_parser = check_path)]
    file: String,

    /// Print the comment during the execution of the program
    #[clap(short, long)]
    verbose: bool,

    /// Threat `Warning` as `Error`
    #[clap(long)]
    werror: bool,
}

pub fn get_path(args: &Args) -> String {
    args.file.clone()
}

pub fn get_verbose(args: &Args) -> bool {
    args.verbose
}

pub fn get_werror(args: &Args) -> bool {
    args.werror
}

pub fn print(args: &Args) {
    println!("{} Arguments:", "|".dimmed());
    println!("{} + {}", "|".dimmed(), "`PATH`".green());
    println!(
        "{} + {}",
        "|".dimmed(),
        if get_verbose(args) {
            "`VERBOSE`".green()
        } else {
            "`VERBOSE`".red()
        }
    );
    println!(
        "{} + {}",
        "|".dimmed(),
        if get_werror(args) {
            "`WERROR`".green()
        } else {
            "`WERROR`".red()
        }
    );
}

pub fn process_args() -> Args {
    let args: Args = Args::parse();

    if get_verbose(&args) {
        print(&args);
    }

    args
}

fn check_path(input: &str) -> Result<String, String> {
    if !Path::new(input).exists() {
        Err(format!("No file exists with this name: `{}`", input))
    } else {
        Ok(input.to_string())
    }
}
