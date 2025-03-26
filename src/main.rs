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
    let path = Path::new(p);
    let target_path = if path.is_absolute() {
        PathBuf::from(path)
    } else {
        env::current_dir().unwrap_or_default().join(path)
    };

    if let Ok(cononicalized) = target_path.canonicalize() {
        if cononicalized.exists() {
            let _ = env::set_current_dir(&cononicalized);
        } else {
            println!("cd: {}: No such file or directory", p);
        }
    } else {
        println!("cd: {}: No such file or directory", p);
    }
}

fn parse(input: String) {
    let args: Vec<&str> = input.split_whitespace().collect();

    if args.is_empty() {
        println!("{}: command not found", input.trim()); // Default all commands invalid
        return;
    }

    match args[0] {
        // Builtin `exit`
        "exit" if args.get(1).map_or(false, |&arg| arg == "0") => std::process::exit(0),
        // Builtin `echo`
        "echo" => {
            println!("{}", args[1..].join(" "));
        },
        // Builtin `type` builtins
        "type" if args.len() > 1 => handle_type_command(&args[1..]),
        // Builtin `pwd`
        "pwd" => {
            println!("{}", env::current_dir().unwrap().display())
        },
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
