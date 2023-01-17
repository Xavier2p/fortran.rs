#[allow(unused_imports)]
use crate::{
    errors::{Error, ErrorKind},
    file_traitement::File,
    preprocess::Args,
    tokens::Token,
    variables::Variable,
};
use std::collections::HashMap;

#[allow(dead_code)]
pub struct Program {
    filename: String,
    path: String,
    name: String,
    variables: HashMap<String, Variable>,
    lines: Vec<Vec<Token>>,
    pc: usize,
    args: Args,
}

impl Program {
    // #[allow(dead_code)]
    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn get_lines(&self) -> &Vec<Vec<Token>> {
        &self.lines
    }

    pub fn get_args(&self) -> &Args {
        &self.args
    }

    pub fn get_variables(&self) -> &HashMap<String, Variable> {
        &self.variables
    }

    pub fn get_filename(&self) -> &String {
        &self.filename
    }

    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn new(
        name: String,
        lines: Vec<Vec<Token>>,
        variables: HashMap<String, Variable>,
        args: Args,
        filename: String,
        path: String,
    ) -> Program {
        Program {
            filename,
            path,
            name,
            variables,
            lines,
            pc: 0,
            args,
        }
    }

    pub fn set_variable(&mut self, name: String, value: Variable) {
        self.variables.remove(&name);
        self.variables.insert(name, value);
    }

    pub fn clone(&self) -> Program {
        Program {
            filename: self.filename.clone(),
            path: self.path.clone(),
            name: self.name.clone(),
            variables: self.variables.clone(),
            lines: self.lines.clone(),
            pc: self.pc,
            args: self.args.clone(),
        }
    }
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
        "PRINT" => return Token::new(Token::Print),
        "PROGRAM" => return Token::new(Token::Program),
        "IF" => return Token::new(Token::If),
        "THEN" => return Token::new(Token::Then),
        "ELSE" => return Token::new(Token::Else),
        "FOR" => return Token::new(Token::For),
        "RETURN" => return Token::new(Token::Return),
        "END" => return Token::new(Token::End),
        "::" | "=" => return Token::new(Token::Assign(word)),
        "REAL" | "INTEGER" | "CHARACTER" | "LOGICAL" => return Token::new(Token::Type(word)),
        "+" | "-" | "*" | "/" => return Token::new(Token::Operator(word)),
        _ => return Token::new(Token::Other(word)),
    };
}

fn parse_line(line: String, _pc: usize) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut tmp_word: String = String::new();
    let mut in_bracket: bool = false;

    for index in 0..line.len() {
        let letter: char = line.chars().nth(index).unwrap();

        if letter == '\"' {
            in_bracket = !in_bracket;

            if tmp_word.len() > 0 {
                tokens.push(Token::new(Token::String(tmp_word)));
                tmp_word = String::new();
            }

            continue;
        }

        if in_bracket {
            tmp_word.push(letter);
        } else {
            if letter == ' ' || letter == ',' || index == line.len() - 1 {
                if index == line.len() - 1 {
                    tmp_word.push(letter);
                }

                if tmp_word.len() > 0 {
                    let mut token: Token = tokenize(tmp_word.clone());

                    if matches!(token, Token::Other(_)) {
                        if tokens.len() > 0 && tokens.last().unwrap() == &Token::Program {
                            token = Token::new(Token::Identifier(tmp_word));
                        } else if tokens.len() > 0
                            && tokens.last().unwrap() == &Token::Assign("::".to_string())
                        {
                            token = Token::new(Token::Variable(tmp_word.clone()));
                            // } else {
                            //     let error = Error::new(
                            //         "tests.f90".to_string(),
                            //         "module".to_string(),
                            //         pc,
                            //         index,
                            //         format!("Unknown token `{}`", tmp_word),
                            //         ErrorKind::UnknownToken,
                            //     );
                            //     error.warn();
                        }
                    }
                    tokens.push(token);
                    tmp_word = String::new();
                }
            } else if letter == '!' {
                let mut comment: String = String::new();

                for index_rest in index..line.len() {
                    let letter_rest: char = line.chars().nth(index_rest).unwrap();
                    comment.push(letter_rest);
                }

                tokens.push(Token::new(Token::Comment(comment)));
                break;
            } else {
                tmp_word.push(letter);
            }
        }
    }

    return tokens;
}

pub fn parser(file: File, args: Args) -> Program {
    let tmp_lines: Vec<String> = split_line(file.clone());

    let mut lines: Vec<Vec<Token>> = Vec::new();

    for index in 0..tmp_lines.len() {
        let line: String = tmp_lines[index].clone();
        let parsed_line: Vec<Token> = parse_line(line, index + 1);

        lines.push(parsed_line);
    }

    let name: String = lines[0][1].get_value().clone();

    Program {
        filename: file.get_name().to_string(),
        path: file.get_path().to_string(),
        name,
        variables: HashMap::new(),
        lines,
        pc: 0,
        args,
    }
}
