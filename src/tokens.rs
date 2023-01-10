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
    Number(i32),
}

impl Token {
    pub fn new(token: Token) -> Self {
        token
    }

    pub fn get_value(&self) -> String {
        return match self {
            Token::Identifier(string) => string.to_string(),
            Token::Comment(string) => string.to_string(),
            Token::String(string) => string.to_string(),
            Token::Operator(string) => string.to_string(),
            Token::Other(string) => string.to_string(),
            Token::Variable(string) => string.to_string(),
            Token::Type(string) => string.to_string(),
            Token::Assign(string) => string.to_string(),
            _ => String::from(""),
        };
    }

    pub fn get_name(&self) -> String {
        return match self {
            Token::Comment(_) => String::from("Comment"),
            Token::String(_) => String::from("String"),
            Token::Operator(_) => String::from("Operator"),
            Token::Identifier(_) => String::from("Identifier"),
            Token::Other(_) => String::from("Other"),
            Token::Variable(_) => String::from("Variable"),
            Token::Type(_) => String::from("Type"),
            Token::Assign(_) => String::from("Assign"),
            Token::Number(_) => String::from("Number"),
            Token::Null => String::from("Null"),
            Token::Print => String::from("Print"),
            Token::Program => String::from("Program"),
            Token::End => String::from("End"),
            Token::For => String::from("For"),
            Token::If => String::from("If"),
            Token::Then => String::from("Then"),
            Token::Else => String::from("Else"),
            Token::Return => String::from("Return"),
        };
    }
}
