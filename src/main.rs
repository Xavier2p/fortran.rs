mod errors;
mod file_traitement;
mod parser;
mod tokens;
mod lexer;
mod variables;

fn main() {
    let file = file_traitement::open_file();
    let program = parser::parser(file);

    for line in program.get_lines() {
        for token in line {
            print!("{} ", token.get_name());
        }
        println!();
    }

    lexer::lexer(program);
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
