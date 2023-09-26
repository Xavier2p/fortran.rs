//! # `fortran.rs`
//!
//! > `fortran.rs` is an interpreter for the Fortran language.
//!
//! ## Usage
//!
//! ```bash
//! $ cargo run -- <file>
//! ```
//!
//! ## Arguments
//!
//! * `<file>` - The path to the file to interpret.
//!
//! ## Options
//!
//! * `-h`, `--help` - Print the help message.
//! * `-v`, `--verbose` - Print the verbose output.
//! * `-V`, `--version` - Print the version.
//! * `--werror` - Treat all warnings as errors.
mod ast;
mod helpers;
mod lexer;
mod parser;
mod program;
mod tokenizer;
mod tokens;
mod utils;
mod validation;
mod variables;
mod verbose;

use crate::helpers::cli;
use clap::Parser;

static VERBOSE: bool = true;

fn main() {
    let args = cli::Cli::parse();

    if !VERBOSE {
        args.debug();
    }

    match args.get_command() {
        cli::Commands::Run { .. } => {
            println!("Running: `{}`...", args.get_path());
        }
        cli::Commands::Check { .. } => {
            println!("Checking: `{}`...", args.get_path());
        }
    }

    let file = helpers::file::new(args);

    if !VERBOSE {
        file.debug();
    }

    let mut program = parser::parse(&file);

    if VERBOSE {
        program.debug();
    }

    program = variables::parse(program);

    lexer::lexer(&mut program);
}
