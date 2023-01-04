use crate::{
    errors::{Error, ErrorKind},
    file_traitement::File,
    tokens::{Token, Tokens},
};
use std::env;
// use std::collections::HashMap;
// use std::io::Split;

#[allow(dead_code)]
pub struct Program {
    name: String,
    // variables: HashMap<String, >,
    lines: Vec<Vec<Token>>,
    pc: usize,
    verbose: bool,
}

impl Program {
    #[allow(dead_code)]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_lines(&self) -> &Vec<Vec<Token>> {
        &self.lines
    }

    pub fn get_verbose(&self) -> bool {
        self.verbose
    }

    // fn clone(&self) -> Program {
    //     Program {
    //         name: self.name.clone(),
    //         lines: self.lines.clone(),
    //         pc: self.pc,
    //     }
    // }
}

pub fn split_line(file: File) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let tmp_lines = file.get_content().split('\n');

    for line in tmp_lines {
        if line.len() != 0 {
            lines.push(line.to_string());
        }
    }

    return lines;
}

fn tokenize(word: String) -> Token {
    match word.to_ascii_uppercase().as_str() {
        "PRINT" => return Token::new(Tokens::Print, word),
        "PROGRAM" => return Token::new(Tokens::Program, word),
        "IF" => return Token::new(Tokens::If, word),
        "THEN" => return Token::new(Tokens::Then, word),
        "ELSE" => return Token::new(Tokens::Else, word),
        "FOR" => return Token::new(Tokens::For, word),
        "RETURN" => return Token::new(Tokens::Return, word),
        "END" => return Token::new(Tokens::End, word),
        "+" | "-" | "*" | "/" => return Token::new(Tokens::Operator, word),
        "*," => return Token::new(Tokens::Other, word),
        _ => return Token::new(Tokens::Null, word),
    };
}

fn parse_line(line: String, pc: usize) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut tmp_word: String = String::new();
    let mut in_bracket: bool = false;

    for index in 0..line.len() {
        let letter = line.chars().nth(index).unwrap();

        if letter == '\"' {
            in_bracket = !in_bracket;

            if tmp_word.len() > 0 {
                // tmp_word.push(letter);
                tokens.push(Token::new(Tokens::String, tmp_word));
                tmp_word = String::new();
            }

            continue;
        }

        if in_bracket {
            tmp_word.push(letter);
        } else {
            if letter == ' ' || index == line.len() - 1 {
                if index == line.len() - 1 {
                    tmp_word.push(letter);
                }

                if tmp_word.len() > 0 {
                    let mut token: Token = tokenize(tmp_word.clone());

                    if token.get_token() == "Null".to_string() {
                        if tokens.last().unwrap().get_token() == "Program".to_string() {
                            token = Token::new(Tokens::Identifier, tmp_word);
                        } else {
                            let error = Error::new(
                                "tests".to_string(),
                                "module".to_string(),
                                pc,
                                index,
                                format!("Unknown token `{}`", tmp_word),
                                ErrorKind::UnknownToken,
                            );
                            error.warn();
                        }
                    }
                    tokens.push(token);
                    tmp_word = String::new();
                }
            } else if letter == '!' {
                let mut comment: String = String::new();

                for index_rest in index..line.len() {
                    let letter_rest = line.chars().nth(index_rest).unwrap();
                    comment.push(letter_rest);
                }

                tokens.push(Token::new(Tokens::Comment, comment));
                break;
            } else {
                tmp_word.push(letter);
            }
        }
    }

    return tokens;
}

pub fn parser(file: File) -> Program {
    let tmp_lines = split_line(file);

    let mut lines: Vec<Vec<Token>> = Vec::new();

    for index in 0..tmp_lines.len() {
        let line = tmp_lines[index].clone();
        let parsed_line = parse_line(line, index + 1);

        lines.push(parsed_line);
    }

    let name: String = lines[0][1].get_value().clone();

    let verbose = if env::args().len() >= 3 {
        let verbose: bool = match env::args().nth(2).unwrap().as_str() {
            "-v" => true,
            _ => false,
        };
        verbose
    } else {
        false
    };

    Program {
        name: name.to_lowercase(),
        lines,
        pc: 0,
        verbose,
    }
}
