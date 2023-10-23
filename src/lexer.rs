//! Lexer Implementation
//!
//! The lexer is the first step of the compilation process. It takes the source code and converts it into tokens.
use crate::{
    fortran::print::print_to_stdout,
    helpers::errors::{self, Error},
    program::Program,
    tokens::Token,
};

/// This function returns the tokens.
pub fn lexer(program: &mut Program) {
    let mut stack: Vec<Token> = Vec::new();

    for pc in 0..program.clone().get_lines().len() {
        let tmp_program: Program = program.clone();
        let line: &Vec<Token> = tmp_program.get_lines().get(pc).unwrap();
        for index in 0..line.len() {
            let token: &Token = line.get(index).unwrap();
            match token {
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
                                stack.last().unwrap().debug().to_ascii_uppercase(),
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
                // Token::Variable(_) => {
                //     *program = variables::assign(line.to_vec(), index, program, token);
                // }
                _ => errors::raise(
                    program,
                    Error::UnexpectedToken,
                    format!("Unexpected token {} `{}`", token.debug(), token.get_value()),
                ),
            }
        }
    }
}
