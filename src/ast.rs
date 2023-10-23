//! Abstract Syntax Tree
//!
//! This module contains the AST and the evaluation function.
use crate::tokens::Token;

#[allow(dead_code)]
pub enum Ast {
    Empty,
    Node {
        token: Token,
        left: Box<Ast>,
        right: Box<Ast>,
    },
}

#[allow(dead_code)]
impl Ast {
    fn new(token: Token, left: Box<Ast>, right: Box<Ast>) -> Ast {
        Ast::Node { token, left, right }
    }
}

#[allow(dead_code)]
#[allow(unused_variables)]
pub fn eval(root: Ast) {
    match root {
        Ast::Empty => {}
        Ast::Node { token, left, right } => match token {
            Token::Variable(_) => {}
            Token::Operator(_) => {}
            _ => {}
        },
    }
}
