//! # Variables
//!
//! This module contains the functions to parse and assign variables.
use crate::{program::Program, tokens::Token};
use std::collections::HashMap;

/// This enum contains the different types of variables.
#[derive(PartialEq, Debug, Clone)]
pub enum Variable {
    Integer(i32),
    Real(f64),
    Character(String),
    Logical(bool),
}

/// This struct contains the variables.
impl Variable {
    /// This function returns a new variable.
    pub fn new_integer(value: i32) -> Variable {
        Variable::Integer(value)
    }

    /// This function returns a new variable.
    pub fn new_real(value: f64) -> Variable {
        Variable::Real(value)
    }

    /// This function returns a new variable.
    pub fn new_character(value: String) -> Variable {
        Variable::Character(value)
    }

    /// This function returns a new variable.
    pub fn new_logical(value: bool) -> Variable {
        Variable::Logical(value)
    }

    /// This function returns the value of the variable.
    pub fn get_value(&self) -> String {
        match self {
            Variable::Integer(value) => value.to_string(),
            Variable::Real(value) => value.to_string(),
            Variable::Character(value) => value.to_string(),
            Variable::Logical(value) => value.to_string(),
        }
    }
}

/// This function parses the variables.
pub fn parse(program: Program) -> Program {
    let mut variables: HashMap<String, Variable> = HashMap::new();
    let mut lines: Vec<Vec<Token>> = Vec::new();

    for pc in 0..program.get_lines().len() {
        let line: &Vec<Token> = &program.get_lines()[pc];
        let mut new_line: Vec<Token> = Vec::new();

        if matches!(line[0], Token::Type(_)) {
            for tok in line {
                match tok {
                    Token::Assign(_) => new_line.push(Token::Assign("::".to_string())),
                    Token::Variable(name) | Token::Other(name) => {
                        let var: Variable = match line[0].get_value().to_ascii_uppercase().as_str()
                        {
                            "INTEGER" => Variable::new_integer(0),
                            "REAL" => Variable::new_real(0.0),
                            "CHARACTER" => Variable::new_character("".to_string()),
                            "LOGICAL" => Variable::new_logical(false),
                            _ => panic!("Not a type = {}", line[0].get_value()),
                        };

                        variables.insert(name.to_string(), var);
                        new_line.push(Token::Variable(name.to_string()));
                    }
                    _ => new_line.push(tok.clone()),
                }
            }
        } else {
            for tok in line {
                match tok {
                    Token::Other(_) => {
                        new_line.push(if variables.contains_key(tok.get_value().as_str()) {
                            Token::Variable(tok.get_value().to_string())
                        } else {
                            tok.clone()
                        });
                    }
                    _ => new_line.push(tok.clone()),
                }
            }
        }

        lines.push(new_line);
    }

    return Program::new(
        program.get_name().to_string(),
        lines,
        variables,
        program.get_args().clone(),
        program.get_filename().to_string(),
    );
}

/// This function assigns the variables.
pub fn assign(line: Vec<Token>, index: usize, program: &mut Program, token: &Token) -> Program {
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
    program.clone()
}
