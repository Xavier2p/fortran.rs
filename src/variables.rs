// use crate::{
    // tokens::{Token, Tokens},
    // errors::{Error, ErrorKind},
    // parser::Program,
// };

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum Variable {
    Integer(i32), // { name: String, value: i32 },
    Real(f64), // { name: String, value: f64 },
    Character(String), // { name: String, value: String },
    Logical(bool), // { name: String, value: bool },
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Variable {
    pub fn new_integer(value: i32) -> Variable {
        Variable::Integer(value)// { name, value }
    }

    pub fn new_real(value: f64) -> Variable {
        Variable::Real(value)// { name, value }
    }

    pub fn new_character(value: String) -> Variable {
        Variable::Character(value)// { name, value }
    }

    pub fn new_logical(value: bool) -> Variable {
        Variable::Logical(value)// { name, value }
    }

    // pub fn get_name(&self) -> String {
    //     match self {
    //         Variable::Integer { name, .. } => name.to_string(),
    //         Variable::Real { name, .. } => name.to_string(),
    //         Variable::Character { name, .. } => name.to_string(),
    //         Variable::Logical { name, .. } => name.to_string(),
    //     }
    // }

    // pub fn get_value_integer(&self) -> i32 {
    //     match self {
    //         Variable::Integer { value, .. } => *value,
    //         _ => panic!("Not an integer"),
    //     }
    // }

    // pub fn get_value_real(&self) -> f64 {
    //     match self {
    //         Variable::Real { value, .. } => *value,
    //         _ => panic!("Not a real"),
    //     }
    // }

    // pub fn get_value_character(&self) -> String {
    //     match self {
    //         Variable::Character { value, .. } => value.to_string(),
    //         _ => panic!("Not a character"),
    //     }
    // }

    // pub fn get_value_logical(&self) -> bool {
    //     match self {
    //         Variable::Logical { value, .. } => *value,
    //         _ => panic!("Not a logical"),
    //     }
    // }
}
