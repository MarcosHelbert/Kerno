pub mod token;

use logos::Logos;
pub use token::Token;

pub type Span = std::ops::Range<usize>;

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken<'a> {
    pub token: Token,
    pub slice: &'a str,
    pub span: Span,
}

pub fn lexer<'a>(source: &'a str) -> Vec<SpannedToken<'a>> {
    let mut lex = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(res) = lex.next() {
        let span = lex.span();
        let slice = &source[span.clone()];

        match res {
            Ok(token) => {
                tokens.push(SpannedToken {
                    token,
                    slice,
                    span,
                });
            }
            Err(err) => {
                // TODO: registrar erro, warning e etc.
            }
        }
    }

    tokens
}
