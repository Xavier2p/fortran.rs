//! # Lexer
//!
//! The lexer is the first step of the compilation process. It takes the source code and converts it into tokens.
use crate::{
    helpers::errors::{self, Error},
    print::print_to_stdout,
    program::Program,
    tokens::Token,
    variables, VERBOSE,
};
use colored::Colorize;

/// This function returns the tokens.
pub fn lexer(program: &mut Program) {
    let mut stack: Vec<Token> = Vec::new();

    for pc in 0..program.clone().get_lines().len() {
        let tmp_program: Program = program.clone();
        let line: &Vec<Token> = tmp_program.get_lines().get(pc).unwrap();
        for index in 0..line.len() {
            let token: &Token = line.get(index).unwrap();
            match token {
                Token::Comment(_) => {
                    if VERBOSE {
                        println!("{} {}", "|".dimmed(), token.get_value().dimmed());
                    }
                }
                Token::Print => {
                    print_to_stdout(line.to_vec(), index, program);
                    break;
                }
                Token::Program => {
                    stack.push(Token::Program);
                }
                Token::End => {
                    if stack.last().unwrap() == line.get(index + 1).unwrap() {
                        stack.pop();
                    } else {
                        errors::raise(
                            program,
                            Error::UnexpectedToken,
                            format!(
                                "Expected `END {}`, got `END {}`",
                                stack.last().unwrap().get_name().to_ascii_uppercase(),
                                line.get(index + 1)
                                    .unwrap()
                                    .get_value()
                                    .to_ascii_uppercase()
                            ),
                        )
                    }
                }
                Token::Identifier(_) | Token::Assign(_) | Token::Other(_) => {}
                Token::Type(_) => break,
                Token::Variable(_) => {
                    *program = variables::assign(line.to_vec(), index, program, token);
                }
                _ => errors::warn(
                    program,
                    Error::UnexpectedToken,
                    format!(
                        "Unexpected token {} `{}`",
                        token.get_name(),
                        token.get_value()
                    ),
                ),
            }
        }
    }
}
