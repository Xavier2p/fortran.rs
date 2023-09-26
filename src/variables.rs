//! # Variables
//!
//! This module contains the functions to parse and assign variables.
use crate::{program::Program, tokens::Token};
use std::collections::HashMap;

/// This enum contains the different types of variables.
#[derive(PartialEq, Debug, Clone)]
pub enum VariableType {
    Integer(Option<i32>),
    Real(Option<f64>),
    Character(Option<String>),
    Logical(Option<bool>),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Variable {
    value: VariableType,
    name: String,
}

impl Variable {
    pub fn new(value: VariableType, name: String) -> Variable {
        Variable { value, name }
    }

    pub fn get_value(&self) -> &VariableType {
        &self.value
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn change_value(&mut self, value: VariableType) {
        self.value = value;
    }
}

/// This struct contains the variables.
impl VariableType {
    /// This function returns a new variable.
    pub fn new_integer(value: i32) -> VariableType {
        VariableType::Integer(Some(value))
    }

    /// This function returns a new variable.
    pub fn new_real(value: f64) -> VariableType {
        VariableType::Real(Some(value))
    }

    /// This function returns a new variable.
    pub fn new_character(value: String) -> VariableType {
        VariableType::Character(Some(value))
    }

    /// This function returns a new variable.
    pub fn new_logical(value: bool) -> VariableType {
        VariableType::Logical(Some(value))
    }

    /// This function returns the value of the variable.
    pub fn get_value(&self) -> String {
        match self {
            VariableType::Integer(value) => value.unwrap().to_string(),
            VariableType::Real(value) => value.unwrap().to_string(),
            VariableType::Character(value) => value.clone().unwrap(),
            VariableType::Logical(value) => value.unwrap().to_string(),
        }
    }
}

/// This function parses the variables.
pub fn parse(program: Program) -> Program {
    let mut variables: HashMap<String, VariableType> = HashMap::new();
    let mut lines: Vec<Vec<Token>> = Vec::new();

    for pc in 0..program.get_lines().len() {
        let line: &Vec<Token> = &program.get_lines()[pc];
        let mut new_line: Vec<Token> = Vec::new();

        if matches!(line[0], Token::Type(_)) {
            for tok in line {
                match tok {
                    Token::Assign(_) => new_line.push(Token::Assign("::".to_string())),
                    Token::Variable(name) | Token::Other(name) => {
                        let var: VariableType =
                            match line[0].get_value().to_ascii_uppercase().as_str() {
                                "INTEGER" => VariableType::new_integer(0),
                                "REAL" => VariableType::new_real(0.0),
                                "CHARACTER" => VariableType::new_character("".to_string()),
                                "LOGICAL" => VariableType::new_logical(false),
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
        program.get_filename().to_string(),
    );
}

/// This function assigns the variables.
pub fn assign(line: Vec<Token>, index: usize, program: &mut Program, token: &Token) -> Program {
    if line.get(index + 1).unwrap() == &Token::Assign("=".to_string()) {
        let new_variable: VariableType = match program
            .clone()
            .get_variables()
            .get_key_value(token.get_value().as_str())
            .unwrap()
            .1
        {
            VariableType::Integer(_) => {
                let value: i32 = line
                    .get(index + 2)
                    .unwrap()
                    .get_value()
                    .parse::<i32>()
                    .unwrap();
                VariableType::Integer(Some(value))
            }
            VariableType::Real(_) => {
                let value: f64 = line
                    .get(index + 2)
                    .unwrap()
                    .get_value()
                    .parse::<f64>()
                    .unwrap();
                VariableType::Real(Some(value))
            }
            VariableType::Character(_) => {
                let value: String = line.get(index + 2).unwrap().get_value();
                VariableType::Character(Some(value))
            }
            VariableType::Logical(_) => {
                let value: bool = line
                    .get(index + 2)
                    .unwrap()
                    .get_value()
                    .parse::<bool>()
                    .unwrap();
                VariableType::Logical(Some(value))
            }
        };

        program.set_variable(token.get_value(), new_variable);
    }
    program.clone()
}
