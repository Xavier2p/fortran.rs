use crate::{
    errors::{Error, ErrorKind},
    print::print_to_stdout,
    program::Program,
    tokens::Token,
    variables,
};
use colored::Colorize;

pub fn lexer(program: &mut Program) {
    let mut stack: Vec<Token> = Vec::new();

    for pc in 0..program.clone().get_lines().len() {
        let tmp_program: Program = program.clone();
        let line: &Vec<Token> = tmp_program.get_lines().get(pc).unwrap();
        for index in 0..line.len() {
            let token: &Token = line.get(index).unwrap();
            match token {
                Token::Comment(_) => {
                    if program.get_args().get_verbose() {
                        println!("{} {}", "|".dimmed(), token.get_value().dimmed());
                    }
                }
                Token::Print => {
                    print_to_stdout(line.to_vec(), index, pc, program.clone());
                    break;
                }
                Token::Program => {
                    stack.push(Token::Program);
                }
                Token::End => {
                    if stack.last().unwrap() == line.get(index + 1).unwrap() {
                        stack.pop();
                    } else {
                        let error: Error = Error::new(
                            program.get_filename().to_string(),
                            program.get_name().to_string(),
                            pc,
                            index,
                            format!(
                                "Expected `END {}`, got `END {}`",
                                stack.last().unwrap().get_name().to_ascii_uppercase(),
                                line.get(index + 1)
                                    .unwrap()
                                    .get_value()
                                    .to_ascii_uppercase()
                            ),
                            ErrorKind::UnexpectedToken,
                        );
                        error.raise();
                    }
                }
                Token::Identifier(_) | Token::Assign(_) | Token::Other(_) => {}
                Token::Type(_) => break,
                Token::Variable(_) => {
                    *program = variables::assign(line.to_vec(), index, program, token);
                }
                _ => {
                    let error: Error = Error::new(
                        program.get_filename().to_string(),
                        program.get_name().to_string(),
                        pc,
                        index,
                        format!(
                            "Unexpected token {} `{}`",
                            token.get_name(),
                            token.get_value()
                        ),
                        ErrorKind::UnexpectedToken,
                    );
                    error.warn();
                }
            }
        }
    }
}
