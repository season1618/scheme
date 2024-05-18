#![no_std]
#![no_main]

extern crate alloc;

mod m5core2;
mod no_std;
mod data;
mod lexer;
mod parser;
mod exec;

use esp32_hal::prelude::entry;
use esp_println::{print, println};
use alloc::{
    string::String,
};

use crate::m5core2::M5Core2;
use crate::no_std::init_heap;
use crate::data::Env;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::exec::{exec, exec_line};

#[entry]
fn main() -> ! {
    init_heap();

    let mut m5core2 = M5Core2::new();

    m5core2.draw("Scheme").unwrap();

    let accel = m5core2.accel();
    let gyro = m5core2.gyro();
    let temp = m5core2.temp();

    println!("accel (g m / s^2): {:5.2?}", accel);
    println!("gyro (degree / s): {:5.2?}", gyro);
    println!("temp  (degree C) : {}", temp);

    interprete(&mut m5core2).unwrap();

    loop {
        repl(&mut m5core2)
    }
}

fn repl(m5core2: &mut M5Core2) {
    let buf: &mut [u8] = &mut [0; 128];
    let mut env = Env::new();
    loop {
        print!("> ");

        let line = m5core2.read_line(buf);
        match line {
            Ok("exit") => { println!("exit"); break; },
            Ok(code) => {
                if let Err(err) = interprete_line(code, &mut env, m5core2) {
                    println!("{err}");
                }
            },
            Err(err) => println!("{err}"),
        }
    }
}

fn interprete(m5core2: &mut M5Core2) -> Result<(), String> {
    let code = "";
    let tokens = tokenize(code)?;
    let nodes = parse(tokens)?;
    exec(nodes, m5core2)
}

fn interprete_line(code: &str, env: &mut Env, m5core2: &mut M5Core2) -> Result<(), String> {
    let tokens = tokenize(code)?;
    if !tokens.is_empty() {
        let nodes = parse(tokens)?;
        for node in nodes {
            exec_line(node, env, m5core2)?;
        }
    }
    Ok(())
}
