//! # Print
//!
//! This module contains the `print` function.
use crate::{
    helpers::errors::{self, Error},
    program::Program,
    tokens::Token,
};

/// This function prints the value of the variable to the standard output.
pub fn print_to_stdout(line: Vec<Token>, index: usize, prog: &Program) {
    if line.get(index + 1).unwrap().get_value() == "*" {
        let mut to_print: String = String::new();
        for index in index + 1..line.len() {
            if matches!(line.get(index).unwrap(), &Token::String(_)) {
                to_print.push_str(line.get(index).unwrap().get_value().as_str());
            } else if matches!(line.get(index).unwrap(), &Token::Variable(_)) {
                to_print.push_str(
                    prog.get_variables()
                        .get_key_value(line.get(index).unwrap().get_value().as_str())
                        .unwrap()
                        .1
                        .get_value()
                        .as_str(),
                );
            }
        }
        println!("{}", to_print);
    } else {
        errors::raise(
            prog,
            Error::Syntax,
            format!(
                "Expected `PRINT *,`, got `PRINT {}`",
                line.get(index + 1).unwrap().get_value()
            ),
        )
    }
}
