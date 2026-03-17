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
    // as ref needed because input becomes &'str???
    match input.as_ref() {
        "exit" => { exit_builtin();
        },
        _ => {
            println!("{}: command not found", input);
        }

    }
}

fn exit_builtin() {
    std::process::exit(0);
}
