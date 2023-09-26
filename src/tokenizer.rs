//! # Tokenizer module
//!
//! This module is in charge of tokenizing the file.
//! It's the first step of the interpreter.
//! Each letter is reviewed, and the primitive tokens are created.
use crate::{helpers::file::File, program::Program, tokens::Token};
use std::collections::HashMap;

/// This enum contains the different types of quotes, and if there is no quote.
#[derive(PartialEq)]
enum InQuote {
    No,
    Single,
    Double,
}

fn new_line(lines: &mut Vec<Vec<Token>>, tmp_line: &mut Vec<Token>, tmp_word: &mut String) {
    new_word(tmp_line, tmp_word, false);
    if !tmp_line.is_empty() {
        lines.push(tmp_line.clone());
    }
    tmp_line.clear();
}

fn new_word(tmp_line: &mut Vec<Token>, tmp_word: &mut String, is_string: bool) {
    if !tmp_word.is_empty() {
        tmp_line.push(if is_string {
            Token::String(tmp_word.clone())
        } else {
            Token::Other(tmp_word.clone())
        });
        tmp_word.clear();
    }
}

pub fn tokenizer(file: &File) -> Program {
    let content: String = file.get_content().clone();
    let mut lines: Vec<Vec<Token>> = Vec::new();
    let mut tmp_line: Vec<Token> = Vec::new();
    let mut tmp_word: String = String::new();
    let mut in_quote: InQuote = InQuote::No;

    for letter in content.chars() {
        match letter {
            '\n' => {
                if in_quote == InQuote::No {
                    new_line(&mut lines, &mut tmp_line, &mut tmp_word);
                }
            }
            '\"' | '\'' => {
                match (&in_quote, letter) {
                    (InQuote::No, '\"') => {
                        in_quote = InQuote::Double;
                        tmp_word = String::new();
                    }
                    (InQuote::No, '\'') => {
                        in_quote = InQuote::Single;
                        tmp_word = String::new();
                    }
                    (InQuote::Double, '\"') | (InQuote::Single, '\'') => {
                        in_quote = InQuote::No;
                        new_word(&mut tmp_line, &mut tmp_word, true);
                    }
                    _ => {
                        tmp_word.push(letter);
                    }
                };
            }
            ' ' => {
                // add support for commas
                if in_quote == InQuote::No {
                    new_word(&mut tmp_line, &mut tmp_word, false);
                } else {
                    tmp_word.push(letter);
                }
            }
            '!' => {
                if in_quote == InQuote::No {
                    new_line(&mut lines, &mut tmp_line, &mut tmp_word);
                } else {
                    tmp_word.push(letter);
                }
            }
            '+' | '-' | '*' | '/' => tmp_line.push(Token::new(Token::Operator(letter.to_string()))),
            '=' => tmp_line.push(Token::new(Token::Assign(letter.to_string()))),
            _ => {
                tmp_word.push(letter);
            }
        };
    }

    Program::new(
        "test".to_string(),
        lines,
        HashMap::new(),
        file.get_path().to_string(),
    )
}
