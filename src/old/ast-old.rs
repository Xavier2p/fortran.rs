use crate::{
    // errors::{Error, ErrorKind},
    tokens::Token,
    variables::Variable,
};

pub struct Node {
    pub token: Token,
    pub left: Box<Node>,
    pub right: Box<Node>,
}

trait NodeTrait<T> {
    fn new(token: Token, left: Box<Node>, right: Box<Node>) -> Node;
    fn eval(&self) -> T;
}

impl NodeTrait<i32> for Node {
    fn new(token: Token, left: Box<Node>, right: Box<Node>) -> Node {
        Node { token, left, right }
    }

    fn eval(&self) -> i32 {
        match self.token {
            Token::Operator(_) => match self.token.get_value().as_str() {
                "+" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left + right
                }
                "-" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left - right
                }
                "*" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left * right
                }
                "/" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left / right
                }
                _ => 0,
            },
            Token::Number(n) => n,
            Token::Variable(v) => {
                let var = Variable::new(v);
                var.get_value() // parse value for i32
            }
            _ => 0,
        }
    }
}

impl NodeTrait<f64> for Node {
    fn new(token: Token, left: Box<Node>, right: Box<Node>) -> Node {
        Node { token, left, right }
    }

    fn eval(&self) -> f64 {
        match self.token {
            Token::Operator(_) => match self.token.get_value().as_str() {
                "+" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left + right
                }
                "-" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left - right
                }
                "*" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left * right
                }
                "/" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left / right
                }
                _ => 0.0,
            },
            Token::Number(n) => n,
            Token::Variable(v) => {
                let var = Variable::new(v);
                var.get_value() // parse value for f64
            }
            _ => 0.0,
        }
    }
}

impl NodeTrait<String> for Node {
    fn new(token: Token, left: Box<Node>, right: Box<Node>) -> Node {
        Node { token, left, right }
    }

    fn eval(&self) -> String {
        match self.token {
            Token::Operator(_) => match self.token.get_value().as_str() {
                "+" => {
                    let left: String = self.left.eval();
                    let right: String = self.right.eval();
                    left + &right
                }
                "-" => {
                    let left: String = self.left.eval();
                    let right: String = self.right.eval();
                    left - right
                }
                "*" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left * right
                }
                "/" => {
                    let left = self.left.eval();
                    let right = self.right.eval();
                    left / right
                }
                _ => "0".to_string(),
            },
            Token::Number(n) => n.to_string(),
            Token::Variable(v) => {
                let var = Variable::new(v);
                var.get_value()
            }
            _ => "0".to_string(),
        }
    }
}

// impl Node {
//     pub fn new(token: Token, left: Box<Node>, right: Box<Node>) -> Node {
//         Node { token, left, right }
//     }

//     pub fn eval(&self) {
//         match self.token {
//             Token::Operator(_) => match self.token.get_value().as_str() {
//                 "+" => {
//                     let left = self.left.eval();
//                     let right = self.right.eval();
//                     left + right
//                 }
//                 "-" => {
//                     let left = self.left.eval();
//                     let right = self.right.eval();
//                     left - right
//                 }
//                 "*" => {
//                     let left = self.left.eval();
//                     let right = self.right.eval();
//                     left * right
//                 }
//                 "/" => {
//                     let left = self.left.eval();
//                     let right = self.right.eval();
//                     left / right
//                 }
//                 _ => {
//                     let error = Error::new(
//                         "test".to_string(),
//                         "module".to_string(),
//                         0,
//                         12,
//                         "Unknown operator".to_string(),
//                         ErrorKind::Syntax,
//                     );
//                     error.raise();
//                 }
//             },
//             Token::Number(n) => n,
//             Token::Variable(v) => {
//                 let var = Variable::new(v);
//                 var.get_value()
//             }
//             _ => 0,
//         }
//     }
// }
