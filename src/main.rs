#![no_std]
#![no_main]

extern crate alloc;

mod m5core2;
mod embedded;
mod data;
mod lexer;
mod parser;
mod exec;

use esp32_hal::prelude::entry;
use esp_println::{print, println};
use alloc::{
    string::String,
};

use crate::m5core2::{
    M5Core2,
    m5core2_new,
    read_line,
    write,
    accel, gyro, temp,
};
use crate::embedded::init_heap;
use crate::data::Env;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::exec::{exec, exec_line};

#[entry]
fn main() -> ! {
    init_heap();

    let mut m5core2 = m5core2_new();

    write(&mut m5core2.lcd, "Scheme").unwrap();

    let accel = accel(&mut m5core2.imu);
    let gyro = gyro(&mut m5core2.imu);
    let temp = temp(&mut m5core2.imu);

    println!("accel (g m / s^2): {:5.2?}", accel);
    println!("gyro (degree / s): {:5.2?}", gyro);
    println!("temp  (degree C) : {}", temp);

    interprete().unwrap();

    loop {
        repl(&mut m5core2).unwrap();
    }
}

fn repl<'a>(m5core2: &mut M5Core2) -> Result<(), String> {
    let buf: &mut [u8] = &mut [0; 128];
    let mut env = Env::new();
    loop {
        print!("> ");

        let line = read_line(&mut m5core2.uart, buf);
        match line {
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
