mod data;
mod lexer;
mod parser;
mod eval;

use std::env;
use std::fs;

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::eval::exec;

fn main() {
    let args: Vec<String> = env::args().collect();
    match interprete(&args[1]) {
        Ok(()) => {},
        Err(err) => eprintln!("{err}"),
    }
}

fn interprete(file: &str) -> Result<(), String> {
    let code = &fs::read_to_string(file).expect("file not found");
    let tokens = tokenize(code)?;
    let nodes = parse(tokens)?;
    exec(nodes)
}
