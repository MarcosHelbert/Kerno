mod lexer;
mod parser;

use std::fs;
use lexer::lexer;

fn main() {
    let source = fs::read_to_string("example.kr").expect("Erro ao ler arquivo");

    let tokens = &source;

    let mut lexer = lexer(tokens);

    println!("Tokens:");
    for token in lexer.iter() {
        println!("{:?}", token);
    }
}
