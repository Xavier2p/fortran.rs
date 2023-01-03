pub enum Tokens {
    Comment,
    String,
    Null,
    Operator,
    Print,
}

pub struct Token {
    token: Tokens,
    value: String,
}

impl Token {
    pub fn new(token: Tokens, value: String) -> Token {
        Token { token, value }
    }

    pub fn get_token(&self) -> &Tokens {
        &self.token
    }

    pub fn get_value(&self) -> &String {
        &self.value
    }
}
