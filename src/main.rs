mod file_traitement;
mod parser;

fn main() {
    // println!("Hello, world!");
    let file = file_traitement::open_file();
    println!("File path: return ./{}", file.get_path());
    println!("File name: {}", file.get_name());
    println!("File version: {}", file.get_version());
    println!("File content:\n{}", file.get_content());

    let program = parser::read_program(file);
    println!("Program name: {}", program.get_name());
    println!("Program lines: {:#?}", program.get_lines());
}
