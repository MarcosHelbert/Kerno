use ast::Expression;

use crate::lexer::{ SpannedToken, Token };

pub mod ast;

pub struct Parser<'a> {
    tokens: &'a [SpannedToken<'a>],
    position: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [SpannedToken<'a>]) -> Self {
        Parser {
            tokens,
            position: 0,
        }
    }
}
