//! Variables definition and assignment
//!
//! This module contains the functions to parse and assign variables.
use crate::tokens::Token;
use std::collections::HashMap;

/// This enum contains the different types of variables.
#[derive(PartialEq, Debug, Clone)]
pub enum Variable {
    /// This variant contains an integer.
    Integer(Option<i32>),

    /// This variant contains a real number.
    Real(Option<f64>),

    /// This variant contains a character.
    Character(Option<String>),

    /// This variant contains a logical value.
    Logical(Option<bool>),
    None,
}

impl Variable {
    /// This function returns the value of the variable.
    /// Else, it returns `[NONE]`.
    pub fn get_value_string(&self) -> String {
        match self {
            Variable::Integer(value) => match value {
                Some(value) => value.to_string(),
                None => "[NONE]".to_string(),
            },
            Variable::Real(value) => match value {
                Some(value) => value.to_string(),
                None => "[NONE]".to_string(),
            },
            Variable::Character(value) => match value {
                Some(value) => value.to_string(),
                None => "[NONE]".to_string(),
            },
            Variable::Logical(value) => match value {
                Some(value) => value.to_string(),
                None => "[NONE]".to_string(),
            },
            Variable::None => "[NONE]".to_string(),
        }
    }
}

/// This function changes the variable in the line.
/// It is used to assign the type of the variable.
pub fn change_in_line(line: &mut Vec<Token>, name: String, position: usize) {
    line.remove(position);
    line.insert(position, Token::Variable(name));
}

/// This function assigns the type of the variable.
/// It is used to assign the type of the variable.
pub fn assign_type(line: &mut Vec<Token>, variables: &mut HashMap<String, Variable>) {
    let kind: Variable = match line[0].get_value().as_str() {
        "INTEGER" => Variable::Integer(None),
        "REAL" => Variable::Real(None),
        "CHARACTER" => Variable::Character(None),
        "LOGICAL" => Variable::Logical(None),
        _ => Variable::None,
    };
    let mut is_assigned: bool = false;
    for i in 1..line.len() {
        match (line[i].clone(), is_assigned) {
            (Token::Assign("::"), false) => is_assigned = true,
            (Token::Other(_), true) => {
                let name = line[i].get_value().to_string();
                change_in_line(line, name.clone(), i);
                variables.insert(name, kind.clone());
            }
            _ => {}
        }
    }
}

// /// This function assigns the variables.
// pub fn assign(line: Vec<Token>, index: usize, program: &mut Program, token: &Token) -> Program {
//     if line.get(index + 1).unwrap() == &Token::Assign("=".to_string()) {
//         let new_variable: VariableType = match program
//             .clone()
//             .get_variables()
//             .get_key_value(token.get_value().as_str())
//             .unwrap()
//             .1
//         {
//             VariableType::Integer(_) => {
//                 let value: i32 = line
//                     .get(index + 2)
//                     .unwrap()
//                     .get_value()
//                     .parse::<i32>()
//                     .unwrap();
//                 VariableType::Integer(Some(value))
//             }
//             VariableType::Real(_) => {
//                 let value: f64 = line
//                     .get(index + 2)
//                     .unwrap()
//                     .get_value()
//                     .parse::<f64>()
//                     .unwrap();
//                 VariableType::Real(Some(value))
//             }
//             VariableType::Character(_) => {
//                 let value: String = line.get(index + 2).unwrap().get_value();
//                 VariableType::Character(Some(value))
//             }
//             VariableType::Logical(_) => {
//                 let value: bool = line
//                     .get(index + 2)
//                     .unwrap()
//                     .get_value()
//                     .parse::<bool>()
//                     .unwrap();
//                 VariableType::Logical(Some(value))
//             }
//         };

//         program.set_variable(token.get_value(), new_variable);
//     }
//     program.clone()
// }
