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

fn parse(input: String) {
    let path = if let Ok(p) = env::var("PATH") {
        p
    } else {
        String::new()
    };

    let args: Vec<&str> = input.split(" ").collect();

    if args.len() > 1 {
        // Builtin `exit`
        if args[0].trim() == "exit" && args[1].trim() == "0" {
            std::process::exit(0)
        }

        // Builtin `echo`
        if args[0].trim() == "echo" {
            print!("{}", args[1..].join(" "));
            return;
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
                let mut path_found = false;
                //println!("{:?}", list_paths);
                for dir in list_paths {
                    let full_path = Path::new(dir).join(cmd);
                    if  full_path.is_file() {
                        println!("{} is {}", cmd, full_path.to_str().unwrap());
                        path_found = true;
                        break;
                    } 
                }
                if !path_found {
                    println!("{}: not found", cmd); // Type not found
                }
            } else {
                println!("{}: not found", cmd); // Type not found
            }
        }
    } else {
        println!("{}: command not found", input.trim()); // Default all commands invalid
    }
}

fn main() {

    // `REPL` Read - Eval - Print - Loop
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        parse(input.clone());
    }
}
