use std::collections::HashMap;
use once_cell::sync::Lazy;

#[allow(unused_imports)]
use std::io::{self, Write};

static BUILTIN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("exit", "exit is a builtin shell"),
        ("echo", "echo is a builtin shell"),
    ])
});

fn main() {
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
            } else {
                println!("{}: command not found", cmd); // Default all commands invalid
            }
            continue;
        }
        
        println!("{}: command not found", input.trim()); // Default all commands invalid
    }
}
