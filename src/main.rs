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
mod errors;
mod file_traitement;
mod lexer;
mod parser;
mod preprocess;
mod print;
mod program;
mod tokens;
mod variables;

fn main() {
    let args = preprocess::process_args();
    let file = file_traitement::File::new(args);
    let mut program = parser::parser(file);

    program = variables::parse(program);

    lexer::lexer(&mut program);
}
