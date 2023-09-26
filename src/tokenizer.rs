use crate::tokens::Token;

pub fn word(tmp_line: &mut Vec<Token>, tmp_word: &mut String, is_string: bool) {
    if !tmp_word.is_empty() {
        tmp_line.push(if is_string {
            Token::String(tmp_word.clone())
        } else {
            match tmp_word.to_ascii_uppercase().as_str() {
                "::" | "=" => Token::Assign(tmp_word.to_string()),
                "DO" => Token::Do,
                "ELSE" => Token::Else,
                "END" => Token::End,
                "IF" => Token::If,
                "PRINT" => Token::Print,
                "PROGRAM" => Token::Program,
                "REAL" | "INTEGER" | "CHARACTER" | "LOGICAL" => Token::Type(tmp_word.to_string()),
                "RETURN" => Token::Return,
                "THEN" => Token::Then,
                "WHILE" => Token::While,
                _ => Token::Other(tmp_word.to_string()),
            }
        });
        tmp_word.clear();
    }
}
