use crate::{
    errors::{Error, ErrorKind},
    parser::Program,
    tokens::Token,
    variables::Variable,
    // variables::Variable,
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
                    if program.get_verbose() {
                        println!("{}", token.get_value().dimmed());
                    }
                }
                Token::Print => {
                    if line.get(index + 1).unwrap().get_value() == "*" {
                        let mut to_print: String = String::new();
                        for index in index + 1..line.len() {
                            if matches!(line.get(index).unwrap(), &Token::String(_)) {
                                to_print.push_str(line.get(index).unwrap().get_value().as_str());
                            } else if matches!(line.get(index).unwrap(), &Token::Variable(_)) {
                                to_print.push_str(
                                    program
                                        .get_variables()
                                        .get_key_value(
                                            line.get(index).unwrap().get_value().as_str(),
                                        )
                                        .unwrap()
                                        .1
                                        .get_value()
                                        .as_str(),
                                );
                            }
                        }
                        println!("{}", to_print);
                        break;
                    } else {
                        let error: Error = Error::new(
                            "tests".to_string(),
                            "module".to_string(),
                            pc,
                            index,
                            format!(
                                "Expected `PRINT *,`, got `PRINT {}`",
                                line.get(index + 1).unwrap().get_value()
                            ),
                            ErrorKind::Syntax,
                        );
                        error.raise();
                    }
                }
                Token::Program => {
                    stack.push(Token::Program);
                }
                Token::End => {
                    if stack.last().unwrap() == line.get(index + 1).unwrap() {
                        stack.pop();
                    } else {
                        let error: Error = Error::new(
                            "tests".to_string(),
                            "module".to_string(),
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
                Token::Identifier(_) | Token::Assign(_) | Token::Other(_) => {
                    // println!("Identifier: {}", token.get_value());
                }
                Token::Type(_) => break,
                Token::Variable(_) => {
                    if line.get(index + 1).unwrap() == &Token::Assign("=".to_string()) {
                        let new_variable: Variable = match program
                            .clone()
                            .get_variables()
                            .get_key_value(token.get_value().as_str())
                            .unwrap()
                            .1
                        {
                            Variable::Integer(_) => {
                                let value: i32 = line
                                    .get(index + 2)
                                    .unwrap()
                                    .get_value()
                                    .parse::<i32>()
                                    .unwrap();
                                Variable::Integer(value)
                            }
                            Variable::Real(_) => {
                                let value: f64 = line
                                    .get(index + 2)
                                    .unwrap()
                                    .get_value()
                                    .parse::<f64>()
                                    .unwrap();
                                Variable::Real(value)
                            }
                            Variable::Character(_) => {
                                let value: String = line.get(index + 2).unwrap().get_value();
                                Variable::Character(value)
                            }
                            Variable::Logical(_) => {
                                let value: bool = line
                                    .get(index + 2)
                                    .unwrap()
                                    .get_value()
                                    .parse::<bool>()
                                    .unwrap();
                                Variable::Logical(value)
                            }
                        };
                        program.set_variable(token.get_value(), new_variable);
                    }
                }
                _ => {
                    let error: Error = Error::new(
                        "tests.f90".to_string(),
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
