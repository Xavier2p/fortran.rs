mod errors;
mod file_traitement;
mod lexer;
mod parser;
mod preprocess;
mod print;
mod program;
mod tokens;
mod variables;

fn main() {
    let args = preprocess::process_args();
    let file = file_traitement::File::new(args);
    let mut program = parser::parser(file);

    program = variables::parse(program);

    lexer::lexer(&mut program);
}
