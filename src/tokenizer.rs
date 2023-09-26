use crate::{
    tokens::Token,
    variables::{self, Variable},
};
use std::collections::HashMap;

pub fn word(tmp_line: &mut Vec<Token>, tmp_word: &mut String, is_string: bool) {
    if !tmp_word.is_empty() {
        tmp_line.push(if is_string {
            Token::String(tmp_word.clone())
        } else {
            match tmp_word.clone().to_ascii_uppercase().as_str() {
                "::" => Token::Assign("::"),
                "DO" => Token::Do,
                "ELSE" => Token::Else,
                "END" => Token::End,
                "IF" => Token::If,
                "PRINT" => Token::Print,
                "PROGRAM" => Token::Program,
                "REAL" | "INTEGER" | "CHARACTER" | "LOGICAL" => {
                    Token::Type(tmp_word.to_string().to_ascii_uppercase())
                }
                "RETURN" => Token::Return,
                "THEN" => Token::Then,
                "WHILE" => Token::While,
                _ => Token::Other(tmp_word.to_string()),
            }
        });
        tmp_word.clear();
    }
}

pub fn variables(lines: &mut [Vec<Token>]) -> HashMap<String, Variable> {
    let mut vars: HashMap<String, Variable> = HashMap::new();

    for line in lines.iter_mut() {
        match line[0] {
            Token::Do | Token::Else | Token::End | Token::If | Token::Program => {
                continue;
            }
            Token::Type(_) => {
                variables::assign_type(line, &mut vars);
            }
            _ => {
                for i in 0..line.len() {
                    if let Token::Other(_) = line[i].clone() {
                        let name = line[i].get_value().to_string();
                        if vars.contains_key(&name) {
                            variables::change_in_line(line, name.clone(), i);
                        }
                    }
                }
            }
        }
    }

    vars
}
