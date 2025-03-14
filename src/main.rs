#[allow(unused_imports)]
use std::io::{self, Write};

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

        if args[0].trim() == "echo" {
            print!("{}", args[1..].join(" "));
        } else {
            println!("{}: command not found", input.trim());
        }
    }
}
