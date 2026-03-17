#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        eval(read());
    }
}

fn read() -> String {
    let mut input = String::new();
    // Need to flush buffer to print $ immediately
    io::stdout().flush().expect("Failed to flush");
    io::stdin().read_line(&mut input).expect("Failed to read line");
    return input.trim().to_string();
}

fn eval(input : String){
    println!("{}: command not found", input);
}
