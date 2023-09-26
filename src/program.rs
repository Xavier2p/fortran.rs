//! # Program
//!
//! This module contains the `Program` struct.
use crate::{tokens::Token, variables::VariableType};
use std::collections::HashMap;

/// This struct contains the program's name, lines, variables, arguments and program counter
#[derive(Clone)]
pub struct Program {
    filename: String,
    name: String,
    variables: HashMap<String, VariableType>,
    lines: Vec<Vec<Token>>,
    pc: u8,
    // implicit: bool,
}

/// This struct contains the program's name, lines, variables, arguments and program counter.
impl Program {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_lines(&self) -> &Vec<Vec<Token>> {
        &self.lines
    }

    pub fn get_variables(&self) -> &HashMap<String, VariableType> {
        &self.variables
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn get_line(&self) -> u8 {
        self.pc
    }

    pub fn new(
        name: String,
        lines: Vec<Vec<Token>>,
        variables: HashMap<String, VariableType>,
        filename: String,
    ) -> Program {
        Program {
            filename,
            name,
            variables,
            lines,
            pc: 0,
        }
    }

    /// This function returns the program counter.
    pub fn set_variable(&mut self, name: String, value: VariableType) {
        self.variables.remove(&name);
        self.variables.insert(name, value);
    }

    pub fn debug(&self) {
        println!("Program {{");
        println!("    name: {}", self.name);
        println!("    filename: {}", self.filename);
        println!("    variables: {:?}", self.variables);
        println!("    pc: {}", self.pc);
        println!("    lines: [");
        for line in self.lines.iter() {
            println!("        {:?}", line);
        }
        println!("    ]");
        println!("}}");
        println!("---- ---- ---- ---- ----");
    }
}
