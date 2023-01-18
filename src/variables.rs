use crate::{parser::Program, tokens::Token};
use std::collections::HashMap;

#[allow(dead_code)]
#[derive(PartialEq, Debug, Clone)]
pub enum Variable {
    Integer(i32),
    Real(f64),
    Character(String),
    Logical(bool),
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Variable {
    pub fn new(value: String) -> Variable {
        match value.parse::<i32>() {
            Ok(value) => Variable::Integer(value),
            Err(_) => match value.parse::<f64>() {
                Ok(value) => Variable::Real(value),
                Err(_) => match value.parse::<bool>() {
                    Ok(value) => Variable::Logical(value),
                    Err(_) => Variable::Character(value),
                },
            },
        }
    }

    pub fn new_integer(value: i32) -> Variable {
        Variable::Integer(value)
    }

    pub fn new_real(value: f64) -> Variable {
        Variable::Real(value)
    }

    pub fn new_character(value: String) -> Variable {
        Variable::Character(value)
    }

    pub fn new_logical(value: bool) -> Variable {
        Variable::Logical(value)
    }

    pub fn get_value(&self) -> String {
        match self {
            Variable::Integer(value) => value.to_string(),
            Variable::Real(value) => value.to_string(),
            Variable::Character(value) => value.to_string(),
            Variable::Logical(value) => value.to_string(),
        }
    }
}

pub fn lex_with_variables(program: Program) -> Program {
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
