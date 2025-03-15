use std::{collections::HashMap, path::Path};
use once_cell::sync::Lazy;
use std::env;

#[allow(unused_imports)]
use std::io::{self, Write};

static BUILTIN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("exit", "exit is a shell builtin"),
        ("echo", "echo is a shell builtin"),
        ("type", "type is a shell builtin"),
    ])
});

fn main() {
    let path = if let Ok(p) = env::var("PATH") {
        p
    } else {
        String::new()
    };

    // `REPL` Read - Eval - Print - Loop
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args: Vec<&str> = input.split(" ").collect();

        // Builtin `exit`
        if args[0].trim() == "exit" && args[1].trim() == "0" {
            break;
        }
        
        // Builtin `echo`
        if args[0].trim() == "echo" {
            print!("{}", args[1..].join(" "));
            continue;
        } 
        
        // Builtin `type` builtins
        if args[0].trim() == "type" {
            let cmd: &str = args[1].trim();
            if BUILTIN.contains_key(cmd) {
                println!("{}", BUILTIN[cmd]);
            }
            // if PATH is provided, make fullpath and see if it exists
            else if !path.is_empty() {
                let list_paths: Vec<&str> = path.split(":").collect();
                for dir in list_paths {
                    let full_path = Path::new(dir).join(cmd);
                    if  full_path.is_file() {
                        println!("{} is {}", cmd, full_path.to_str().unwrap());
                        break;
                    }
                }
            } else {
                println!("{}: not found", cmd); // Type not found
            }
            continue;
        }
        
        println!("{}: command not found", input.trim()); // Default all commands invalid
    }
}
