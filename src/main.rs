mod lexer;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};

use crate::lexer::lexer::{LexToken, is_letter};

fn main() {
    let file = File::open("./input.aqri").unwrap();
    let buf = BufReader::new(file);
    let mut lexTokens: Vec<LexToken> = Vec::new();
    for line in buf.lines() {
        // let line = line.unwrap();
        // for i in 0..line.len() {
        //     let char = &line[i..i + 1];
        //     println!("{}", char);
        // }
        let mut lex = lexer::lexer::Lexer::new(line.unwrap());
        lexTokens.append(&mut lex.create_tokens());
    }
    println!("{:?}", lexTokens);
}
