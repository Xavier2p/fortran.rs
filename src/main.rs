//! > Fortran interpreter, written in Rust.
//!
//! Now, it supports only the Fortran90 version, but you can help me to add more versions !
//!
//! ## How to install it
//!
//! ### From source
//!
//! ```bash
//! git clone https://github.com/xavier2p/fortran.rs.git && cd fortran.rs/
//! cargo install --path .
//! ```
//!
//! Other ways are possible, please check [install.md](https://github.com/xavier2p/fortran.rs/blob/main/docs/install.md) for more information.
//!
//! ## How to use it
//!
//! ```bash
//! fortran-rs run <FILE>
//! ```
//!
//! All the options are available with the `--help` option.
//!
//! ```console
//! $ fortran-rs --help
//! An open-source Fortran interpreter.
//! Written in Rust.
//!
//! Usage: fortran-rs [OPTIONS] <COMMAND>
//!
//! Commands:
//!   run    Run the Fortran file passed as argument
//!   check  Check the syntax of the file passed as argument
//!   help   Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -v, --verbose  Print the comment during the execution of the program
//!   -h, --help     Print help
//!   -V, --version  Print version
//! ```
mod ast;
mod fortran;
mod helpers;
mod lexer;
mod parser;
mod program;
mod tokenizer;
mod tokens;
mod validation;
mod variables;
mod verbose;

use crate::helpers::cli;
use clap::Parser;

static VERBOSE: bool = true;

/// Program entry point
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

    // program = variables::parse(program);

    // if VERBOSE {
    //     program.debug();
    // }

    lexer::lexer(&mut program);
}
