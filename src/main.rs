mod lexer;

use std::env;
use std::fs;

use crate::lexer::tokenize;

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
    println!("{:?}", tokens);
}
