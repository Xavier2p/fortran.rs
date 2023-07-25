//! # Program
//!
//! This module contains the `Program` struct.
use crate::{preprocess, tokens::Token, variables::Variable};
use std::collections::HashMap;

/// This struct contains the program's name, lines, variables, arguments and program counter.
pub struct Program {
    filename: String,
    path: String,
    name: String,
    variables: HashMap<String, Variable>,
    lines: Vec<Vec<Token>>,
    pc: u8,
    args: preprocess::Args,
}

/// This struct contains the program's name, lines, variables, arguments and program counter.
impl Program {
    /// This function returns the program counter.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// This function returns the program counter.
    pub fn get_lines(&self) -> &Vec<Vec<Token>> {
        &self.lines
    }

    /// This function returns the program counter.
    pub fn get_args(&self) -> &preprocess::Args {
        &self.args
    }

    /// This function returns the program counter.
    pub fn get_variables(&self) -> &HashMap<String, Variable> {
        &self.variables
    }

    /// This function returns the program counter.
    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    /// This function returns the program counter.
    #[allow(dead_code)]
    pub fn get_path(&self) -> &String {
        &self.path
    }

    /// This function returns the program counter.
    pub fn new(
        name: String,
        lines: Vec<Vec<Token>>,
        variables: HashMap<String, Variable>,
        args: preprocess::Args,
        filename: String,
    ) -> Program {
        Program {
            filename,
            path: preprocess::get_path(&args),
            name,
            variables,
            lines,
            pc: 0,
            args,
        }
    }

    /// This function returns the program counter.
    pub fn set_variable(&mut self, name: String, value: Variable) {
        self.variables.remove(&name);
        self.variables.insert(name, value);
    }

    /// This function returns the program counter.
    pub fn clone(&self) -> Program {
        Program {
            filename: self.filename.clone(),
            path: self.path.clone(),
            name: self.name.clone(),
            variables: self.variables.clone(),
            lines: self.lines.clone(),
            pc: self.pc,
            args: self.args.clone(),
        }
    }
}
