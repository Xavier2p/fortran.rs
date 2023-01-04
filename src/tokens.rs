pub enum Tokens {
    Comment,
    String,
    Null,
    Operator,
    Print,
    Program,
    End,
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
        };
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}
