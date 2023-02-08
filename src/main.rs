// use crate::variables::{Var, Variable};

mod ast;
mod errors;
mod file_traitement;
mod lexer;
mod parser;
mod preprocess;
mod print;
mod program;
mod tests;
mod tokens;
mod variables;

fn main() {
    let args = preprocess::process_args();
    let file = file_traitement::File::new(args);
    let mut program = parser::parser(file);

    program = variables::parse(program);

    lexer::lexer(&mut program);
}

// fn test<T, U>(var: T) -> U
// where
//     T: variables::Var<U>,
//     U: std::fmt::Display,
// {
//     println!("{} => {}", var.get_name(), var.get_value());
//     var.get_value()
// }
