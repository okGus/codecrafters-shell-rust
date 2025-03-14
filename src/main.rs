#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let args: Vec<&str> = input.split(" ").collect();
        if args[0].trim() == "exit" && args[1].trim() == "0" {
            break;
        }

        println!("{}: command not found", input.trim());
    }
}
