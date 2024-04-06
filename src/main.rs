use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let code = fs::read_to_string(&args[1]).expect("file not found");
    println!("{}", code);
}
