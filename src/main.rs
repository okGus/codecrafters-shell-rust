use std::{collections::HashMap, path::{Path, PathBuf}, process::Command};
use once_cell::sync::Lazy;
use std::env;

#[allow(unused_imports)]
use std::io::{self, Write};

static BUILTIN: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    HashMap::from([
        ("exit", "exit is a shell builtin"),
        ("echo", "echo is a shell builtin"),
        ("type", "type is a shell builtin"),
        ("pwd", "pwd is a shell builtin"),
        ("cd", "cd is a shell builtin"),
    ])
});

fn handle_type_command(args: &[&str]) {
    let cmd = args[0];
    if let Some(builtin_desc) = BUILTIN.get(cmd) {
        println!("{}", builtin_desc);
        return;
    }

    if let Some(path) = env::var_os("PATH") {
        for dir in env::split_paths(&path) {
            let full_path = dir.join(cmd);
            if full_path.is_file() {
                println!("{} is {}", cmd, full_path.display());
                return;
            }
        }
    }

    println!("{}: not found", cmd);
}

fn handle_cd_command(p: &str) {
    // This should also handle relative, absolute, and ~
    let target_path = if p.eq("~") {
        dirs::home_dir().unwrap_or_else(|| PathBuf::from("/"))
    } else {
        let path = Path::new(p);
        if path.is_absolute() {
            PathBuf::from(path)
        }
        else {
            env::current_dir().unwrap_or_default().join(path)
        }
    }; 

    if let Ok(cononicalized) = target_path.canonicalize() {
        if cononicalized.exists() {
            let _ = env::set_current_dir(&cononicalized);
            return;
        } else {
            println!("cd: {}: No such file or directory", p);
        }
    } else {
        println!("cd: {}: No such file or directory", p);
    }
}

fn handle_echo_command(args: &[&str]) {
    println!("{}", args.join(" "));
}

fn process_input(input: &str) -> Vec<String> {
    // Handles input for different commands by
    // processing each character
    // from escaping `\'` or `\"` or `\ `
    // to single, double quotes
    let mut result: Vec<String> = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut quote_char = '\0';
    let mut escape_char = false;
    let chars: Vec<char> = input.chars().collect();
    
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];

        if escape_char {
            if in_quotes && c == quote_char {
                current.push(c);
            } else {
                //current.push('\\');
                current.push(c);
            }
            escape_char = false;
        } else {
            match c {
                '\\' => {
                    if i + 1 < chars.len() {
                        escape_char = true;
                    } else {
                        current.push('\\');
                    }
                },
                '"' | '\'' => {
                    if in_quotes {
                        if c == quote_char {
                            in_quotes = false;
                        } else {
                            current.push(c);
                        }
                    } else {
                        in_quotes = true;
                        quote_char = c;
                    }
                },
                ' ' => {
                    if !in_quotes {
                        if !current.is_empty() {
                            result.push(std::mem::take(&mut current));
                        }
                    } else {
                        current.push(c);
                    }
                },
                _ => current.push(c),
            } // end match
        } // end else
        //println!("idx:{}", i);
        //println!("char:{}", c);
        //println!("current:{}", current);
        //println!("result:{:?}\n", result);
        i += 1;
    } // end while
    if escape_char {
        current.push('\\');
    }

    if !current.is_empty() {
        result.push(current);
    }

    result
}

fn parse(input: String) {
    let processed_input: Vec<String> = process_input(input.as_str());
    let args: Vec<&str> = processed_input.iter().map(|s| s.trim()).collect();

    if args.is_empty() {
        println!("{}: command not found", input.trim()); // Default all commands invalid
        return;
    }
    match args[0] {
        // Builtin `exit`
        "exit" if args.len() > 1 && args[1] == "0" => std::process::exit(0),
        // Builtin `echo`
        "echo" if args.len() > 1 => handle_echo_command(&args[1..]), 
        // Builtin `type` builtins
        "type" if args.len() > 1 => handle_type_command(&args[1..]),
        // Builtin `pwd`
        "pwd" => {
            println!("{}", env::current_dir().unwrap().display())
        },
        // Builtin `cd`
        "cd" if args.len() >= 1 => handle_cd_command(args[1]),
        // External commands
        cmd => {
            if let Some(path) = env::var_os("PATH") {
                for dir in env::split_paths(&path) {
                    let full_path = dir.join(cmd);
                    if full_path.exists() {
                        if let Ok(output) = Command::new(cmd).args(&args[1..]).output() {
                            print!("{}", String::from_utf8_lossy(&output.stdout));
                        }
                        return;
                    }
                }
            }
            println!("{}: command not found", cmd);
        },
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
