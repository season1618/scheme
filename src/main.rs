#![no_std]
#![no_main]

extern crate alloc;

mod embedded;
mod data;
mod lexer;
mod parser;
mod exec;

use esp32_hal::{
    prelude::entry,
};
use alloc::{
    string::String,
};

use crate::embedded::init_heap;
use crate::lexer::tokenize;
use crate::parser::parse;
use crate::exec::exec;

#[entry]
fn main() -> ! {
    init_heap();

    interprete().unwrap();
    loop {}
}

fn interprete() -> Result<(), String> {
    let code = "";
    let tokens = tokenize(code)?;
    let nodes = parse(tokens)?;
    exec(nodes)
}
