mod lexer;
mod parser;
mod eval;

use std::env;
use std::fs;

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::eval::eval;

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
    let res = match eval(nodes) {
        Ok(res) => res,
        Err(err) => {
            eprintln!("{}", err);
            return;
        },
    };
    println!("{}", res);
}
