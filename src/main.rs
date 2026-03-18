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
    let inputs: Vec<&str> = input.split_whitespace().collect();
    // as ref needed because input becomes &'str???
    match inputs[0].as_ref() {
        "exit" => { exit_builtin();
        },
        "echo" => {echo_builtin(inputs)},
        _ => {
            println!("{}: command not found", input);
        }

    }
}

fn exit_builtin() {
    std::process::exit(0);
}

fn echo_builtin(inputs : Vec<&str>) {
    if let Some((_head, tail)) = inputs.split_first() {
        // .join because &str cant be printed???
        println!("{}", tail.join(" "));
    } else {
        println!("");
    }
}
