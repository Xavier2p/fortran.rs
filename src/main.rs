mod errors;
mod file_traitement;
mod parser;
mod tokens;

fn main() {
    // println!("Hello, world!");
    let file = file_traitement::open_file();
    // println!("File path: return ./{}", file.get_path());
    // println!("File name: {}", file.get_name());
    // println!("File version: {}", file.get_version());
    // println!("File content:\n{}", file.get_content());

    let program = parser::parser(file);

    println!("Program name: {}, ", program.get_name());

    for parsed_line in program.get_lines() {
        for token in parsed_line {
            print!("{} = {}, ", token.get_token(), token.get_value());
        }
        println!();
    }
    // println!("Program name: {}", program.get_name());
    // println!("Program lines: [");
    // for line in program.get_lines() {
    // println!("    {:?}", line);
    // }
    // println!("]");

    // let error = errors::Error::new(
    //     "hello-world.f90".to_string(),
    //     "hello".to_string(),
    //     12,
    //     13,
    //     "bad indentation".to_string(),
    //     2,
    // );
    // error.raise();
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         assert_eq!(2 + 2, 4);
//     }
// }
