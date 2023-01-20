use crate::{preprocess::Args, tokens::Token, variables::Variable};
use std::collections::HashMap;

pub struct Program {
    filename: String,
    path: String,
    name: String,
    variables: HashMap<String, Variable>,
    lines: Vec<Vec<Token>>,
    pc: u8,
    args: Args,
}

impl Program {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_lines(&self) -> &Vec<Vec<Token>> {
        &self.lines
    }

    pub fn get_args(&self) -> &Args {
        &self.args
    }

    pub fn get_variables(&self) -> &HashMap<String, Variable> {
        &self.variables
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    #[allow(dead_code)]
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn new(
        name: String,
        lines: Vec<Vec<Token>>,
        variables: HashMap<String, Variable>,
        args: Args,
        filename: String,
    ) -> Program {
        Program {
            filename,
            path: args.get_path().to_string(),
            name,
            variables,
            lines,
            pc: 0,
            args,
        }
    }

    pub fn set_variable(&mut self, name: String, value: Variable) {
        self.variables.remove(&name);
        self.variables.insert(name, value);
    }

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
