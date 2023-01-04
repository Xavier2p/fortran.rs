use crate::{
    errors::{Error, ErrorKind},
    parser::Program,
    tokens::Token,
    // variables::Variable,
};
use colored::Colorize;

pub fn lexer(program: Program) {
    let mut stack: Vec<Token> = Vec::new();

    for pc in 0..program.get_lines().len() {
        let line = program.get_lines().get(pc).unwrap();
        for index in 0..line.len() {
            let token = line.get(index).unwrap();
            match token {
                Token::Comment(_) => {
                    if program.get_verbose() {
                        println!("{}", token.get_value().dimmed());
                    }
                }
                Token::Print => {
                    if line.get(index + 1).unwrap().get_value() == "*," {
                        let mut to_print = String::new();
                        for index in index + 1..line.len() {
                            if matches!(line.get(index).unwrap(), &Token::String(_)) {
                                to_print.push_str(line.get(index).unwrap().get_value().as_str());
                            }
                        }
                        println!("{}", to_print);
                        break;
                    } else {
                        let error = Error::new(
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
                        let error = Error::new(
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
                // Token::Type(_) => {
                    // if line.get(index).unwrap() == &Token::Assign("::".to_string()) {
                        // match token.get_value().as_str() {
                            // "INTEGER" => Variable::new_integer(, 0),
                        // }
                    // }
                // }
                // "For" => {
                //     println!("For: {}", token.get_value());
                // }
                // "If" => {
                //     println!("If: {}", token.get_value());
                // }
                // "Then" => {
                //     println!("Then: {}", token.get_value());
                // }
                // "Else" => {
                //     println!("Else: {}", token.get_value());
                // }
                Token::Identifier(_) => {
                    // println!("Identifier: {}", token.get_value());
                }
                // "Return" => {
                //     println!("Return: {}", token.get_value());
                // }
                _ => {
                    println!("Unknown token: {}", token.get_value());
                }
            }
        }
    }
}
