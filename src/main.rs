mod errors;
mod file_traitement;
mod lexer;
mod parser;
mod tokens;
mod variables;
// mod ast;
mod preprocess;
mod tests;

fn main() {
    let args = preprocess::process_args();
    let file = file_traitement::File::new(args.get_path_str());
    let mut program = parser::parser(file, args);

    program = variables::lex_with_variables(program);

    lexer::lexer(&mut program);
}

