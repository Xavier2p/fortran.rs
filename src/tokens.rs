#[allow(dead_code)]
pub enum Tokens {
    Comment,
    String,
    Null,
    Operator,
    Print,
    Program,
    End,
    For,
    If,
    Then,
    Else,
    Identifier,
    Return,
    Other,
}

pub struct Token {
    token: Tokens,
    value: String,
}

impl Token {
    pub fn new(token: Tokens, value: String) -> Token {
        Token { token, value }
    }

    pub fn get_token(&self) -> String {
        match self.token {
            Tokens::Comment => return String::from("Comment"),
            Tokens::String => return String::from("String"),
            Tokens::Null => return String::from("Null"),
            Tokens::Operator => return String::from("Operator"),
            Tokens::Print => return String::from("Print"),
            Tokens::Program => return String::from("Program"),
            Tokens::End => return String::from("End"),
            Tokens::For => return String::from("For"),
            Tokens::If => return String::from("If"),
            Tokens::Then => return String::from("Then"),
            Tokens::Else => return String::from("Else"),
            Tokens::Identifier => return String::from("Identifier"),
            Tokens::Return => return String::from("Return"),
            Tokens::Other => return String::from("Other"),
        };
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}
