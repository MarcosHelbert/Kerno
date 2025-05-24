mod lexer;
mod ast;

use std::fs;

use lexer::lexer;

fn main() {
    let source = fs::read_to_string("example.kr").expect("Erro ao ler arquivo");

    let tokens = lexer(&source);
    for token in &tokens {
        println!("{:?}", token);
    }

    // match parser.parse(lexer) {
    //     Ok(ast) => println!("{:#?}", ast),
    //     Err(err) => eprintln!("Erro de parsing: {:?}", err),
    // }
}
