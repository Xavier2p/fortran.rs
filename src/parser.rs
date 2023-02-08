use crate::variables::Var;
#[allow(unused_imports)]
use crate::{
    errors::{Error, ErrorKind},
    file_traitement::File,
    preprocess::Args,
    program::Program,
    tokens::Token,
    variables::Variable,
};
use std::collections::HashMap;

fn split_line(file: File) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let tmp_lines = file.get_content().split('\n');

    for line in tmp_lines.filter(|line| (*line).len() != 0) {
        lines.push(line.to_string());
    }

    return lines;
}

fn tokenize(word: String) -> Token {
    match word.to_ascii_uppercase().as_str() {
        "PRINT" => Token::new(Token::Print),
        "PROGRAM" => Token::new(Token::Program),
        "IF" => Token::new(Token::If),
        "THEN" => Token::new(Token::Then),
        "ELSE" => Token::new(Token::Else),
        "FOR" => Token::new(Token::For),
        "RETURN" => Token::new(Token::Return),
        "END" => Token::new(Token::End),
        "::" | "=" => Token::new(Token::Assign(word)),
        "REAL" | "INTEGER" | "CHARACTER" | "LOGICAL" => Token::new(Token::Type(word)),
        "+" | "-" | "*" | "/" => Token::new(Token::Operator(word)),
        _ => Token::new(Token::Other(word)),
    }
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

                if tmp_word.len() <= 0 {
                    continue;
                }

                let mut token: Token = tokenize(tmp_word.clone());

                if matches!(token, Token::Other(_)) {
                    if tokens.len() > 0 && tokens.last().unwrap() == &Token::Program {
                        token = Token::new(Token::Identifier(tmp_word));
                    } else if tokens.len() > 0
                        && tokens.last().unwrap() == &Token::Assign("::".to_string())
                    {
                        let tmp_var = Variable::new(tmp_word.clone(), None);
                        token = Token::new(Token::Variable(tmp_var));
                        // } else {
                        //     let error = Error::new(
                        //         "tests.f90".to_string(),
                        //         "module".to_string(),
                        //         pc,
                        //         index,
                        //         format!("Unknown token `{}`", tmp_word),
                        //         ErrorKind::UnknownToken,
                        //     );
                    }
                }
                tokens.push(token);
                tmp_word = String::new();
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

pub fn parser(file: File) -> Program {
    let tmp_lines: Vec<String> = split_line(file.clone());
    let mut lines: Vec<Vec<Token>> = Vec::new();

    for index in 0..tmp_lines.len() {
        let line: String = tmp_lines[index].clone();
        let parsed_line: Vec<Token> = parse_line(line, index + 1);

        lines.push(parsed_line);
    }

    let name: String = lines[0][1].get_value().clone();

    Program::new(
        name,
        lines,
        HashMap::new(),
        file.get_args().clone(),
        file.get_name().to_string(),
    )
}
