#[allow(unused_imports)]
use std::io::{self, Write};
use std::fs;
use is_executable::IsExecutable;


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
        "echo" => {echo_builtin(inputs);},
        "type" => {let _ = type_builtin(inputs);},
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

fn type_builtin(inputs : Vec<&str>) -> io::Result<()> {
    // Check if the command is builtin
    let inbuilt_commands = ["echo", "exit", "type"];
    if let Some(input) = inputs.get(1){
        if inbuilt_commands.contains(&input){
            println!("{} is a shell builtin", input);}
        else {
            // Check if there is an external command
            match env::var_os("PATH"){
                Some(paths) => {
                    for path in env::split_paths(&paths) {
                        for entry in fs::read_dir(&path)?{
                            let entry = entry?;
                            let entry_path = entry.path();
                            if entry_path.is_file() && entry_path.file_name().map_or(false, |name| name == *input) && entry_path.is_executable() {
                                println!("{} is {}", input, entry_path.display());
                                return Ok(());
                                //Ok::<(), E>(());
                            }

                        }
                    }

                }
            None => {println!("Could not determine PATH");}
            }
            println!("{}: not found", input);
        }
    }
    Ok(())
}
