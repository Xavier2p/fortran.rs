use crate::variables::{Var, VarType};

#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum Token {
    Null,
    Print,
    Program,
    End,
    For,
    If,
    Then,
    Else,
    Return,
    Comment(String),
    String(String),
    Operator(String),
    Identifier(String),
    Other(String),
    Variable(VarType),
    Type(String),
    Assign(String),
}

impl Token {
    pub fn new(token: Token) -> Self {
        token
    }

    pub fn get_value(&self) -> String {
        match self {
            Token::Identifier(string) => string,
            Token::Comment(string) => string,
            Token::String(string) => string,
            Token::Operator(string) => string,
            Token::Other(string) => string,
            Token::Type(string) => string,
            Token::Assign(string) => string,
            Token::Variable(var_type) => &match var_type {
                VarType::Integer(var) => var.get_name(),
                VarType::Character(var) => var.get_name(),
                VarType::Logical(var)  => var.get_name(),
                VarType::Real(var) => var.get_name(),
            },
            _ => "",
        }
        .to_string()
    }

    pub fn get_token(&self) -> String {
        match self {
            Token::Comment(_) => "COMMENT",
            Token::String(_) => "STRING",
            Token::Operator(_) => "OPERATOR",
            Token::Identifier(_) => "IDENTIFIER",
            Token::Other(_) => "OTHER",
            Token::Variable(_) => "VARIABLE",
            Token::Type(_) => "TYPE",
            Token::Assign(_) => "ASSIGN",
            Token::Null => "NULL",
            Token::Print => "PRINT",
            Token::Program => "PROGRAM",
            Token::End => "END",
            Token::For => "FOR",
            Token::If => "IF",
            Token::Then => "THEN",
            Token::Else => "ELSE",
            Token::Return => "RETURN",
        }
        .to_string()
    }
}
