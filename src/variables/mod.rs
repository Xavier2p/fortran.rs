use crate::{program::Program, tokens::Token};
use std::collections::HashMap;

mod character;
mod integer;
mod logical;
mod real;

#[derive(PartialEq, Clone)]
pub enum VarType {
    Integer(Variable<i32>),
    Real(Variable<f64>),
    Character(Variable<String>),
    Logical(Variable<bool>),
}

#[derive(PartialEq, Clone)]
pub struct Variable<T> {
    name: String,
    value: T,
}

pub trait Var<T> {
    fn new(name: String, value: T) -> Self;
    fn get_value(&self) -> T;
    fn get_name(&self) -> String;
    fn set_value(&mut self, value: T);
}

pub fn parse(program: Program) -> Program {
    let mut variables: HashMap<String, VarType> = HashMap::new();
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
    return program.clone();
}
