mod errors;
mod file_traitement;
mod parser;
mod tokens;
mod lexer;
mod variables;

fn main() {
    let file = file_traitement::open_file();
    let mut program = parser::parser(file);

    program = variables::lex_with_variables(program);

    lexer::lexer(&mut program);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
