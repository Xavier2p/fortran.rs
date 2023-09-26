//! # Parser module
//!
//! This module is in charge of parsing the file.
//! It's the first step of the interpreter.
//! Each letter is reviewed, and the primitive tokens are created.
use crate::{helpers::file::File, program::Program, tokenizer, tokens::Token};

/// This enum contains the different types of quotes, and if there is no quote.
#[derive(PartialEq)]
enum InQuote {
    No,
    Single,
    Double,
}

/// This function creates a new line.
/// It's called when a new line is found.
/// It adds the current line to the lines vector.
/// It clears the current line.
/// It clears the current word.
fn new_line(lines: &mut Vec<Vec<Token>>, tmp_line: &mut Vec<Token>, tmp_word: &mut String) {
    tokenizer::word(tmp_line, tmp_word, false);
    if !tmp_line.is_empty() {
        lines.push(tmp_line.clone());
    }
    tmp_line.clear();
}

/// This function manages the quotes.
/// It's called when a quote is found.
/// It adds the current word to the current line.
/// It clears the current word.
/// It changes the quote state.
fn quote_management(
    in_quote: &mut InQuote,
    letter: char,
    word: &mut String,
    line: &mut Vec<Token>,
) {
    match (&in_quote, letter) {
        (InQuote::No, '\"') => {
            *in_quote = InQuote::Double;
            word.clear();
        }
        (InQuote::No, '\'') => {
            *in_quote = InQuote::Single;
            word.clear();
        }
        (InQuote::Double | InQuote::Single, '\"' | '\'') => {
            *in_quote = InQuote::No;
            tokenizer::word(line, word, true);
        }
        _ => {
            word.push(letter);
        }
    };
}

/// This function parses the file.
pub fn parse(file: &File) -> Program {
    let content: String = file.get_content().clone();
    let mut lines: Vec<Vec<Token>> = Vec::new();
    let mut tmp_line: Vec<Token> = Vec::new();
    let mut tmp_word: String = String::new();
    let mut in_quote: InQuote = InQuote::No;
    let mut in_comment: bool = false;

    for letter in content.chars() {
        match (letter, in_comment, &in_quote) {
            ('\n', true, _) => {
                tmp_word.clear();
                new_line(&mut lines, &mut tmp_line, &mut tmp_word);
                in_comment = false;
            }
            ('\n', false, InQuote::No) => {
                new_line(&mut lines, &mut tmp_line, &mut tmp_word);
            }
            (_, true, _) => tmp_word.push(letter),
            ('\"' | '\'', _, _) => {
                quote_management(&mut in_quote, letter, &mut tmp_word, &mut tmp_line)
            }
            (_, _, InQuote::Double | InQuote::Single) => tmp_word.push(letter),
            (' ', _, _) => tokenizer::word(&mut tmp_line, &mut tmp_word, false),
            (',', _, _) => {
                tokenizer::word(&mut tmp_line, &mut tmp_word, false);
                tmp_line.push(Token::Comma);
            }
            ('!', _, _) => in_comment = true,
            ('+' | '-' | '*' | '/' | '=', _, _) => {
                tmp_line.push(Token::Operator(letter.to_string()))
            }
            _ => tmp_word.push(letter),
        };
    }

    new_line(&mut lines, &mut tmp_line, &mut tmp_word);

    let variables = tokenizer::variables(&mut lines);

    Program::new(
        "test".to_string(),
        lines,
        variables,
        file.get_path().to_string(),
    )
}
