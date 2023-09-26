//! # Tokens
//!
//! This module contains the Token enum, which is used to represent the tokens

/// This enum contains the different types of tokens.
#[derive(Clone, Debug, PartialEq)]
#[allow(dead_code)]
pub enum Token {
    Null,
    Print,
    Program,
    End,
    Do,
    While,
    If,
    Then,
    Else,
    Return,
    Comma,
    Comment(String),
    String(String),
    Operator(String),
    Identifier(String),
    Other(String),
    Variable(String),
    Type(String),
    Assign(String),
    Number(i32),
}

/// This struct contains the tokens.
impl Token {
    /// This function returns a new token.
    pub fn new(token: Token) -> Self {
        token
    }

    /// This function returns the value of the token.
    pub fn get_value(&self) -> String {
        match self {
            Token::Identifier(string) => string,
            Token::Comment(string) => string,
            Token::String(string) => string,
            Token::Operator(string) => string,
            Token::Other(string) => string,
            Token::Variable(var) => var,
            Token::Type(string) => string,
            Token::Assign(string) => string,
            _ => "",
        }
        .to_string()
    }

    /// This function returns the name of the token.
    pub fn debug(&self) -> String {
        match self {
            Token::Assign(_) => "Assign",
            Token::Comment(_) => "Comment",
            Token::Identifier(_) => "Identifier",
            Token::Number(_) => "Number",
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
