mod lexer;
mod parser;

use std::env;
use std::fs;

use crate::lexer::tokenize;
use crate::parser::parse;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = fs::read_to_string(&args[1]).expect("file not found");
    let tokens = match tokenize(&code) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err);
            return;
        },
    };
    let nodes = match parse(tokens) {
        Ok(nodes) => nodes,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    println!("{:?}", nodes);
}
