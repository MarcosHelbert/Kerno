use ariadne::{ Color, Config, Label, Report, ReportKind, Source };

mod lexer;
mod parser;

use std::fs;
use lexer::{ lexer, Span };
use parser::{ error::ParseError, Parser };

fn main() {
    let source = fs::read_to_string("example.kr").expect("Erro ao ler arquivo");
    let tokens = lexer(&source);
    // println!("Tokens:");
    // for token in lexer.iter() {
    //     println!("{:?}", token);
    // }
    let mut parser = Parser::new(&tokens);
    // println!("Parser: {:#?}", parser);
}
