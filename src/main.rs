#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    print!("$ ");
    // Need to flush buffer to print $ immediately
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    println!("{}: command not found", input.trim());
}
