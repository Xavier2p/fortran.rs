use crate::tokens::Token;

#[allow(dead_code)]
struct Node {
    token: Token,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    #[allow(dead_code)]
    fn new(token: Token, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Node {
        Node { token, left, right }
    }
}
