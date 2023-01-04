// use crate::{
// tokens::{Token, Tokens},
// errors::{Error, ErrorKind},
// parser::Program,
// };

#[allow(dead_code)]
pub enum VariableKind {
    Integer { name: String, value: i32 },
    Real { name: String, value: f64 },
    Character { name: String, value: String },
    Logical { name: String, value: bool },
}

#[allow(dead_code)]
impl VariableKind {
    pub fn new_integer(name: String, value: i32) -> VariableKind {
        VariableKind::Integer { name, value }
    }

    pub fn new_real(name: String, value: f64) -> VariableKind {
        VariableKind::Real { name, value }
    }

    pub fn new_character(name: String, value: String) -> VariableKind {
        VariableKind::Character { name, value }
    }

    pub fn new_logical(name: String, value: bool) -> VariableKind {
        VariableKind::Logical { name, value }
    }

    pub fn get_name(&self) -> String {
        match self {
            VariableKind::Integer { name, .. } => name.to_string(),
            VariableKind::Real { name, .. } => name.to_string(),
            VariableKind::Character { name, .. } => name.to_string(),
            VariableKind::Logical { name, .. } => name.to_string(),
        }
    }

    pub fn get_value_integer(&self) -> i32 {
        match self {
            VariableKind::Integer { value, .. } => *value,
            _ => panic!("Not an integer"),
        }
    }

    pub fn get_value_real(&self) -> f64 {
        match self {
            VariableKind::Real { value, .. } => *value,
            _ => panic!("Not a real"),
        }
    }

    pub fn get_value_character(&self) -> String {
        match self {
            VariableKind::Character { value, .. } => value.to_string(),
            _ => panic!("Not a character"),
        }
    }

    pub fn get_value_logical(&self) -> bool {
        match self {
            VariableKind::Logical { value, .. } => *value,
            _ => panic!("Not a logical"),
        }
    }
}
