use crate::file_traitement::File;
// use std::collections::HashMap;
// use std::io::Split;

pub struct Program {
    name: String,
    // variables: HashMap<String, >,
    lines: Vec<Vec<String>>,
}

impl Program {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_lines(&self) -> &Vec<Vec<String>> {
        &self.lines
    }
}

pub fn split_line(file: File) -> Vec<Vec<String>> {
    let mut lines: Vec<Vec<String>> = Vec::new();
    let tmp_lines = file.get_content().split('\n');

    for line in tmp_lines {
        let lexed_line = lex_line(line.to_string());
        lines.push(lexed_line);
    }

    return lines;
}

fn lex_line(line: String) -> Vec<String> {
    let mut words: Vec<String> = Vec::new();
    let mut tmp_word: String = String::new();
    let mut in_bracket: bool = false;

    for letter in line.chars() {
        if letter == '\"' {
            in_bracket = !in_bracket;

            if tmp_word.len() > 0 {
                tmp_word.push(letter);
                words.push(tmp_word);
                tmp_word = String::new();
                continue;
            }
        }

        if in_bracket {
            tmp_word.push(letter);
        } else {
            if letter == ' ' {
                if tmp_word.len() > 0 {
                    words.push(tmp_word);
                    tmp_word = String::new();
                }
            } else {
                tmp_word.push(letter);
            }
        }
    }

    words.push(tmp_word);

    return words;
}

// fn upper_line(line: Split<char>) {
//     let mut in_bracket: bool = false;

//     for word in line {
//         for letter in word {
//             if letter == '\"' {
//                 in_bracket = !in_bracket;
//             }

//             if !in_bracket {
//                 letter.to_uppercase();
//             }
//         }
//     }
// }

fn check_program(lines: Vec<Vec<String>>) {
    //-> bool {
    let first_line = &lines[0]; //.split(' ');
    let last_line = &lines[lines.len() - 1]; //.split(' ');

    println!("{:#?}", first_line);
    println!("{:?}", last_line);

    // upper_line(first_line);
    // upper_line(last_line);

    // return first_line[0] == "PROGRAM" && last_line[0] == "END" && last_line[1] == "PROGRAM"
    // return true;
}

pub fn read_program(file: File) -> Program {
    let lines = split_line(file);

    let tmp_lines = lines.clone();

    check_program(tmp_lines);

    // let name: &String = &lines[0][1]; //.split(' ').last().unwrap().to_string();

    let program = Program {
        name: "name".to_string(),
        // variables: HashMap::new(),
        lines: lines,
    };

    return program;
}
