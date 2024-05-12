extern crate alloc;

mod data;
mod lexer;
mod parser;
mod exec;

use alloc::{
    string::String,
};

use crate::lexer::tokenize;
use crate::parser::parse;
use crate::exec::exec;

fn main() {
    interprete().unwrap();
}

fn interprete() -> Result<(), String> {
    let code = "";
    let tokens = tokenize(code)?;
    let nodes = parse(tokens)?;
    exec(nodes)
}
