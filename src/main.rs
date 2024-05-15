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
    prelude::entry,
};
use esp_println::println;
use alloc::{
    string::String,
};

use crate::m5core2::{m5core2_new, read_line};
use crate::embedded::init_heap;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::exec::exec;

#[entry]
fn main() -> ! {
    init_heap();

    let mut uart = m5core2_new();

    interprete().unwrap();

    let buf: &mut [u8] = &mut [0; 128];
    loop {
        let msg = read_line(&mut uart, buf);
        println!("{}", msg);
    }
}

fn interprete() -> Result<(), String> {
    let code = "";
    let tokens = tokenize(code)?;
    let nodes = parse(tokens)?;
    exec(nodes)
}
