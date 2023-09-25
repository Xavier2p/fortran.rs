use crate::tokens::Token;

struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(token: Token, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node { token, left, right }
    }
}
