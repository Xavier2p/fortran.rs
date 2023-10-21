//! # Program
//!
//! This module contains the `Program` struct.
use crate::{tokens::Token, variables::Variable};
use std::collections::HashMap;

/// This struct is the schema of the `program` object.
#[derive(Clone)]
pub struct Program {
    /// This field contains the program's path.
    filename: String,

    /// This field contains the program's name.
    name: String,

    /// This field contains the program's variables.
    variables: HashMap<String, Variable>,

    /// This field contains the program's lines.
    lines: Vec<Vec<Token>>,

    /// This field contains the program's program counter.
    pc: u8,
    // /// This field contains if the program is implicit.
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

    pub fn get_variables(&self) -> &HashMap<String, Variable> {
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
        variables: HashMap<String, Variable>,
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

    // pub fn add_variable(&mut self, variable: Variable) {
    // self.variables.push(variable);
    // }

    // pub fn remove_variable(&mut self, variable: Variable) {
    // self.variables
    // .remove(self.variables.iter().position(|x| *x == variable).unwrap());
    // }

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
