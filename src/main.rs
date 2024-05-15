#![no_std]
#![no_main]

extern crate alloc;

mod m5core2;
mod embedded;
mod data;
mod lexer;
mod parser;
mod exec;

use esp32_hal::{
    peripherals::UART0,
    prelude::entry,
    uart::Uart,
};
use esp_println::{print, println};
use alloc::{
    string::String,
};

use crate::m5core2::{m5core2_new, read_line, write};
use crate::embedded::init_heap;
use crate::data::Env;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::exec::{exec, exec_line};

#[entry]
fn main() -> ! {
    init_heap();

    let (mut uart, mut lcd) = m5core2_new();

    write(&mut lcd, "Scheme").unwrap();

    interprete().unwrap();

    loop {
        repl(&mut uart).unwrap();
    }
}

fn repl<'a>(uart: &mut Uart<'a, UART0>) -> Result<(), String> {
    let buf: &mut [u8] = &mut [0; 128];
    let mut env = Env::new();
    loop {
        print!("> ");

        match read_line(uart, buf) {
            Ok("") => break Ok(()),
            Ok(code) => {
                let tokens = tokenize(&code)?;
                if tokens.len() == 0 {
                    continue;
                } else {
                    let nodes = parse(tokens)?;
                    for node in nodes {
                        match exec_line(node, &mut env) {
                            Ok(()) => {},
                            Err(err) => println!("{err}"),
                        }
                    }
                }
            },
            Err(err) => println!("{err}"),
        }
    }
}

fn interprete() -> Result<(), String> {
    let code = "";
    let tokens = tokenize(code)?;
    let nodes = parse(tokens)?;
    exec(nodes)
}
