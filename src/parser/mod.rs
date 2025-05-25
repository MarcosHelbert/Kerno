pub mod ast;
pub mod error;

use ast::Expression;
use error::ParseError;

use crate::lexer::{ Span, SpannedToken, Token };

#[derive(Debug)]
pub struct Parser<'a> {
    tokens: &'a [SpannedToken<'a>],
    position: usize,
    last_span: Option<Span>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [SpannedToken<'a>]) -> Self {
        Parser {
            tokens,
            position: 0,
            last_span: None,
        }
    }

    fn is_ignorable(token: &Token) -> bool {
        matches!(token, Token::Whitespace | Token::Comment | Token::BlockComment)
    }

    pub fn peek(&self) -> Option<&Token> {
        self.peek_n(0)
    }

    pub fn peek_n(&self, n: usize) -> Option<&Token> {
        let mut pos = self.position;
        let mut seen = 0;

        while let Some(tok) = self.tokens.get(pos) {
            if !Self::is_ignorable(&tok.token) {
                if seen == n {
                    return Some(&tok.token);
                }
                seen += 1;
            }
            pos += 1;
        }

        None
    }

    pub fn current(&self) -> Option<&SpannedToken<'a>> {
        let mut pos = self.position;
        while let Some(tok) = self.tokens.get(pos) {
            if !Self::is_ignorable(&tok.token) {
                return Some(tok);
            }
            pos += 1;
        }
        None
    }

    pub fn next(&mut self) -> Option<&SpannedToken<'a>> {
        while let Some(tok) = self.tokens.get(self.position) {
            self.position += 1;
            if !Self::is_ignorable(&tok.token) {
                self.last_span = Some(tok.span.clone());
                return Some(tok);
            }
        }
        None
    }

    pub fn span(&self) -> Option<Span> {
        self.current()
            .map(|tok| tok.span.clone())
            .or_else(|| self.last_span.clone())
    }

    pub fn expect(&mut self, expected: Token) -> Result<SpannedToken<'a>, ParseError> {
        match self.current() {
            Some(tok) if tok.token == expected => {
                let tok = self.next().unwrap();
                Ok(tok.clone())
            }
            Some(tok) =>
                Err(
                    ParseError::new(
                        format!("Expected {:?}, found {:?}", expected, tok.token),
                        Some(tok.span.clone())
                    )
                ),
            None => Err(ParseError::new("Unexpected end of input", self.span())),
        }
    }
}
