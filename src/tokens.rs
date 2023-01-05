#[derive(PartialEq, Clone)]
#[allow(dead_code)]
pub enum Token {
    Comment(String),
    String(String),
    Null,
    Operator(String),
    Print,
    Program,
    End,
    For,
    If,
    Then,
    Else,
    Identifier(String),
    Return,
    Other(String),
    Variable(String),
    Type(String),
    Assign(String),
}

impl Token {
    pub fn new(token: Token) -> Self {
        token
    }

    pub fn get_value(&self) -> String {
        match self {
            Token::Identifier(string) => return string.to_string(),
            Token::Comment(string) => return string.to_string(),
            Token::String(string) => return string.to_string(),
            Token::Operator(string) => return string.to_string(),
            Token::Other(string) => return string.to_string(),
            Token::Variable(string) => return string.to_string(),
            Token::Type(string) => return string.to_string(),
            Token::Assign(string) => return string.to_string(),
            _ => return String::from(""),
        };
    }

    pub fn get_name(&self) -> String {
        match self {
            Token::Comment(_) => return String::from("Comment"),
            Token::String(_) => return String::from("String"),
            Token::Operator(_) => return String::from("Operator"),
            Token::Identifier(_) => return String::from("Identifier"),
            Token::Other(_) => return String::from("Other"),
            Token::Variable(_) => return String::from("Variable"),
            Token::Type(_) => return String::from("Type"),
            Token::Assign(_) => return String::from("Assign"),
            Token::Null => return String::from("Null"),
            Token::Print => return String::from("Print"),
            Token::Program => return String::from("Program"),
            Token::End => return String::from("End"),
            Token::For => return String::from("For"),
            Token::If => return String::from("If"),
            Token::Then => return String::from("Then"),
            Token::Else => return String::from("Else"),
            Token::Return => return String::from("Return"),
        };
    }
}
