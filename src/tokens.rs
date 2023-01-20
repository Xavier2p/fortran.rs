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
    Variable(String),
    Type(String),
    Assign(String),
    Number(i32),
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
            Token::Variable(string) => string,
            Token::Type(string) => string,
            Token::Assign(string) => string,
            _ => "",
        }
        .to_string()
    }

    pub fn get_name(&self) -> String {
        match self {
            Token::Comment(_) => "Comment",
            Token::String(_) => "String",
            Token::Operator(_) => "Operator",
            Token::Identifier(_) => "Identifier",
            Token::Other(_) => "Other",
            Token::Variable(_) => "Variable",
            Token::Type(_) => "Type",
            Token::Assign(_) => "Assign",
            Token::Number(_) => "Number",
            Token::Null => "Null",
            Token::Print => "Print",
            Token::Program => "Program",
            Token::End => "End",
            Token::For => "For",
            Token::If => "If",
            Token::Then => "Then",
            Token::Else => "Else",
            Token::Return => "Return",
        }
        .to_string()
    }
}
