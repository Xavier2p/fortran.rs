use crate::tokens::{Token, Tokens};
use crate::{
    errors::{Error, ErrorKind},
    file_traitement::File,
};
// use std::collections::HashMap;
// use std::io::Split;

pub struct Program {
    name: String,
    // variables: HashMap<String, >,
    lines: Vec<Vec<String>>,
    pc: usize,
}

impl Program {
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_lines(&self) -> &Vec<Vec<String>> {
        &self.lines
    }

    fn clone(&self) -> Program {
        Program {
            name: self.name.clone(),
            lines: self.lines.clone(),
            pc: self.pc,
        }
    }
}

pub fn split_line(file: File) -> Vec<Vec<String>> {
    let mut lines: Vec<Vec<String>> = Vec::new();
    let tmp_lines = file.get_content().split('\n');

    for line in tmp_lines {
        let lexed_line = lex_line_string(line.to_string());
        lines.push(lexed_line);
    }

    return lines;
}

#[allow(unused_assignments)]
fn tokenize_line(line: Vec<String>) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut tmp_token: Token = Token::new(Tokens::Null, String::new());

    for index in 0..line.len() {
        let word = line[index].clone();
        if word.len() == 0 {
            continue;
        }

        if word == "PRINT" {
            tmp_token = Token::new(Tokens::Print, word);
            tokens.push(tmp_token);
        } else if word == "PROGRAM" {
            tmp_token = Token::new(Tokens::Program, word);
            tokens.push(tmp_token);
        // } else if word == "IF" {
        //     tmp_token = Token::new(Tokens::If, word);
        // } else if word == "THEN" {
        //     tmp_token = Token::new(Tokens::Then, word);
        // } else if word == "ELSE" {
        //     tmp_token = Token::new(Tokens::Else, word);
        // } else if word == "ENDIF" {
        //     tmp_token = Token::new(Tokens::Endif, word);
        // } else if word == "FOR" {
        //     tmp_token = Token::new(Tokens::For, word);
        // } else if word == "RETURN" {
        //     tmp_token = Token::new(Tokens::Return, word);
        } else if word == "!" {
            let mut comment: String = String::new();

            for index_rest in index..line.len() {
                let word_rest = line[index_rest].clone();
                comment.push_str(word_rest.as_str());
                comment.push(' ')
            }

            tmp_token = Token::new(Tokens::Comment, comment);
            tokens.push(tmp_token);
            break;
        } else if word == "END" {
            tmp_token = Token::new(Tokens::End, word);
            tokens.push(tmp_token);
        } else if word.chars().next().unwrap() == '\"' {
            tmp_token = Token::new(Tokens::String, word);
            tokens.push(tmp_token);
        }
        else {
            println!("{}", word.chars().next().unwrap().to_string()); 
        }
    }

    return tokens;
}

fn lex_line_string(line: String) -> Vec<String> {
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
                tmp_word.push(letter.to_ascii_uppercase());
            }
        }
    }

    words.push(tmp_word);
    return words;
}

// fn format_program(lines: Vec<Vec<String>>) -> Vec<Vec<String>> {
//     let mut formatted_lines: Vec<Vec<String>> = Vec::new();

//     for line in lines {
//         let tmp_line: Vec<String> = upper_line(&mut line.clone());
//         let mut formatted_line = Vec::new();

//         for word in tmp_line {
//             if word.len() > 0 {
//                 formatted_line.push(word);
//             }
//         }

//         if formatted_line.len() > 0 {
//             formatted_lines.push(formatted_line);
//         }
//     }

//     return formatted_lines;
// }

// fn upper_line(line: &mut Vec<String>) -> Vec<String> {
//     let mut capitalized_line: Vec<String> = Vec::new();

//     for word in line {
//         if !word.contains('\"') {
//             let tmp = word.to_uppercase();
//             capitalized_line.push(tmp);
//         } else {
//             capitalized_line.push(word.to_string());
//         }
//     }

//     return capitalized_line;
// }

#[allow(dead_code)]
#[allow(unused_mut)]
fn check_program(mut lines: Vec<Vec<String>>) -> bool {
    let len = lines.len() - 1;
    let first_line = lines[0].clone();
    let last_line = lines[len].clone();

    return first_line[0] == "PROGRAM" && last_line[0] == "END" && last_line[1] == "PROGRAM";
}

#[allow(dead_code)]
fn analyze(program: Program) {
    let mut pc: usize = 1;
    let lines = program.get_lines().clone();

    for line in lines {
        if line[0] == "PRINT" {
            if line[1] != "*," {
                let error = Error::new(
                    program.name.clone(),
                    "module".to_string(),
                    pc,
                    6,
                    "missing `*,` after `PRINT` statement".to_string(),
                    ErrorKind::Syntax,
                );
                error.raise();
            } else {
                println!("PRINT statement found at line {}", pc);
                println!("{}", line[2]);
            }
        }

        pc += 1;
    }
}

pub fn read_program(file: File) -> Program {
    let tmp_lines = split_line(file);

    // let lines = format_program(tmp_lines);

    let name = &tmp_lines[0][1]; //.split(' ').last().unwrap().to_string();

    let program = Program {
        name: name.to_lowercase(),
        // variables: HashMap::new(),
        lines: tmp_lines,
        pc: 0,
    };

    for line in program.lines.clone() {
        let tmp_tokenized_line = tokenize_line(line);
        for token in tmp_tokenized_line {
            println!("{} => {}", token.get_token(), token.get_value());
        }
        println!("BREAK LINE");
    }

    // analyze(program.clone());

    return program;
}
