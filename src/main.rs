mod lexer;
mod parser;
mod statement_parser;
mod utils;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::lexer::lexer::LexToken;

fn main() {
    let file = File::open("./input.aqri").unwrap();
    let buf = BufReader::new(file);
    let mut lex_tokens: Vec<LexToken> = Vec::new();
    for line in buf.lines() {
        let mut lex = lexer::lexer::Lexer::new(line.unwrap());
        lex_tokens.append(&mut lex.create_tokens());
    }
    lex_tokens.push(LexToken {
        symbol: lexer::lexer::SYMBOL::A_EOF,
        val: "EOF".to_string(),
    });
    let mut parser = parser::parser::Parser::new(lex_tokens);
    parser.parse();
}
