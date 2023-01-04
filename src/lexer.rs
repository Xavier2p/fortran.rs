use colored::Colorize;

use crate::{
    // tokens::{Token, Tokens},
    errors::{Error, ErrorKind},
    parser::Program,
};

pub fn lexer(program: Program) {
    let mut stack: Vec<String> = Vec::new();

    for pc in 0..program.get_lines().len() {
        let line = program.get_lines().get(pc).unwrap();
        for index in 0..line.len() {
            let token = line.get(index).unwrap();
            match token.get_token().as_str() {
                "Comment" => {
                    if program.get_verbose() {
                        println!("{}", token.get_value().dimmed());
                    }
                }
                // "String" => {
                //     println!("String: {}", token.get_value());
                // }
                "Print" => {
                    if line
                        .get(index + 1)
                        .unwrap()
                        .get_value()
                        == "*,"
                    {
                        let mut to_print = String::new();
                        for index in index + 1..line.len() {
                            if line.get(index).unwrap().get_token() == "String" {
                                to_print.push_str(line.get(index).unwrap().get_value());
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
                                line.get(index + 1)
                                    .unwrap()
                                    .get_value()
                            ),
                            ErrorKind::Syntax,
                        );
                        error.raise();
                    } 
                }
                "Program" => {
                    stack.push("PROGRAM".to_string());
                }
                "End" => {
                    if *stack.last().unwrap()
                        == line
                            .get(index + 1)
                            .unwrap()
                            .get_value()
                            .to_ascii_uppercase()
                    {
                        stack.pop();
                    } else {
                        let error = Error::new(
                            "tests".to_string(),
                            "module".to_string(),
                            pc,
                            index,
                            format!(
                                "Expected `END {}`, got `END {}`",
                                stack.last().unwrap().to_ascii_uppercase(),
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
                "Identifier" => {
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
