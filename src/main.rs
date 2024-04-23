mod data;
mod lexer;
mod parser;
mod eval;

use std::io::{stdin, stdout, Write};
use std::env;
use std::fs;

use crate::data::Env;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::eval::{exec, exec_line};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        match repl() {
            Ok(()) => {},
            Err(err) => eprintln!("{err}"),
        }
    } else {
        match interprete(&args[1]) {
            Ok(()) => {},
            Err(err) => eprintln!("{err}"),
        }
    }
}

fn interprete(file: &str) -> Result<(), String> {
    let code = &fs::read_to_string(file).expect("file not found");
    let tokens = tokenize(code)?;
    let nodes = parse(tokens)?;
    exec(nodes)
}

fn repl() -> Result<(), String> {
    let mut code;
    let mut env = Env::new();
    loop {
        print!("> ");
        stdout().flush().unwrap();

        code = String::new();
        match stdin().read_line(&mut code) {
            Ok(0) => break Ok(()),
            Ok(_) => {
                let tokens = tokenize(&code)?;
                if tokens.len() == 0 {
                    continue;
                } else {
                    let nodes = parse(tokens)?;
                    for node in nodes {
                        match exec_line(node, &mut env) {
                            Ok(()) => {},
                            Err(err) => eprintln!("{err}"),
                        }
                    }
                }
            },
            Err(err) => eprintln!("{err}"),
        }
    }
}
