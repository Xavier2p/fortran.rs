//! # Tokens
//!
//! This module contains the Token enum, which is used to represent the tokens

/// This enum contains the different types of tokens.
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Assign(&'static str),
    Identifier(String),
    Operator(String),
    Other(String),
    String(String),
    Type(String),
    Variable(String),
    Comma,
    Do,
    Else,
    End,
    If,
    Null,
    Print,
    Program,
    Return,
    Then,
    While,
}

/// This struct contains the tokens.
impl Token {
    /// This function returns the value of the token.
    pub fn get_value(&self) -> String {
        match self {
            Token::Assign(string) => *string,
            Token::Identifier(string) => string,
            Token::Operator(string) => string,
            Token::Other(string) => string,
            Token::String(string) => string,
            Token::Type(string) => string,
            Token::Variable(string) => string,
            _ => "",
        }
        .to_string()
    }

    /// This function returns the name of the token.
    pub fn debug(&self) -> String {
        match self {
            Token::Assign(_) => "Assign",
            Token::Identifier(_) => "Identifier",
            Token::Operator(_) => "Operator",
            Token::Other(_) => "Other",
            Token::String(_) => "String",
            Token::Type(_) => "Type",
            Token::Variable(_) => "Variable",
            Token::Do => "For",
            Token::Else => "Else",
            Token::End => "End",
            Token::If => "If",
            Token::Null => "Null",
            Token::Print => "Print",
            Token::Program => "Program",
            Token::Return => "Return",
            Token::Then => "Then",
            Token::While => "While",
            Token::Comma => "Comma",
        }
        .to_string()
    }
}
