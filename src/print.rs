use crate::{
    errors::{Error, ErrorKind},
    program::Program,
    tokens::Token,
    variables::{Var, VarType},
};

pub fn print_to_stdout(line: Vec<Token>, index: usize, pc: usize, prog: Program) {
    if line.get(index + 1).unwrap().get_value() == "*" {
        let mut to_print: String = String::new();
        for index in index + 1..line.len() {
            if matches!(line.get(index).unwrap(), &Token::String(_)) {
                to_print.push_str(line.get(index).unwrap().get_value().as_str());
            } else if matches!(line.get(index).unwrap(), &Token::Variable(_)) {
                let tmp_str = match *prog
                    .get_variables()
                    .get_key_value(line.get(index).unwrap().get_value().as_str())
                    .unwrap()
                    .1
                {
                    VarType::Character(var) => var.get_value(),
                    VarType::Integer(var) => var.get_value().to_string(),
                    VarType::Real(var) => var.get_value().to_string(),
                    VarType::Logical(var) => var.get_value().to_string(),
                };
                to_print.push_str(&tmp_str);
            }
        }
        println!("{}", to_print);
    } else {
        let error: Error = Error::new(
            prog.get_filename().to_string(),
            prog.get_name().to_string(),
            pc,
            index,
            format!(
                "Expected `PRINT *,`, got `PRINT {}`",
                line.get(index + 1).unwrap().get_value()
            ),
            ErrorKind::Syntax,
        );
        error.raise();
    }
}
