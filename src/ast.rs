use crate::tokens::Token;

pub struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(token: Token, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node { token, left, right }
    }
}

pub fn eval(root: Node) {
    match root.token {
        Token::Variable(_) => {}
        Token::Operator(_) => {}
        _ => {}
    }
}
