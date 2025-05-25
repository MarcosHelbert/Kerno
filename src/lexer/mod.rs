pub mod token;

use logos::Logos;
pub use token::Token;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Span {
    pub start: Position,
    pub end: Position,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SpannedToken<'a> {
    pub token: Token,
    pub slice: &'a str,
    pub span: Span,
}

fn byte_to_position(source: &str, byte_index: usize) -> Position {
    let mut line = 1;
    let mut col = 1;
    let mut count = 0;

    for c in source.chars() {
        if count == byte_index {
            break;
        }

        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }

        count += c.len_utf8();
    }

    Position { line, column: col }
}

pub fn lexer<'a>(source: &'a str) -> Vec<SpannedToken<'a>> {
    let mut lex = Token::lexer(source);
    let mut tokens = Vec::new();

    while let Some(res) = lex.next() {
        let span = lex.span();
        let slice = &source[span.clone()];

        let start_pos = byte_to_position(source, span.start);
        let end_pos = byte_to_position(source, span.end);

        match res {
            Ok(token) => {
                tokens.push(SpannedToken {
                    token,
                    slice,
                    span: Span { start: start_pos, end: end_pos },
                });
            }
            Err(err) => {
                // TODO: registrar erro, warning e etc.
            }
        }
    }

    tokens
}
