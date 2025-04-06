# Rust Shell

[![progress-banner](https://backend.codecrafters.io/progress/shell/0b9a4e7b-05d4-48e6-b320-bc1f13de461d)](https://app.codecrafters.io/users/codecrafters-bot?r=2qF)

**Note**: If you're viewing this repo on GitHub, head over to
[codecrafters.io](https://codecrafters.io) to try the challenge.

## Description

This program is a shell implementation built with Rust. It provides a basic command-line interface, allowing you to execute commands like `exit`, `echo`, `type`, `pwd`, and `cd`.

## Features

*   **Basic Command Execution:** Supports executing built-in commands and external programs.
*   **Built-in Commands:**
    *   `exit`: Exits the shell.
    *   `echo`: Prints arguments to standard output.
    *   `type`: Displays information about a command.
    *   `pwd`: Prints the current working directory.
    *   `cd`: Changes the current working directory.
*   **Command Parsing:**  Handles spaces, quotes, and backslashes within commands.
*   **Path Resolution:** Uses the `PATH` environment variable to locate external commands.

## Installation

1.  Clone this repository: `git clone [repository_url]`
2.  Navigate to the project directory: `cd [project_directory]`
3.  Run the program: `./your_program.sh`

## Usage

The program is interactive. You can type commands and press Enter to execute them.

**Example:**

```
$ pwd
/home/user
$ echo "Hello, world!"
Hello, world!
$ cd ..
$ pwd
/home
$ exit
```

## Installation

1.  Ensure you have Rust and Cargo installed. You can find instructions on the official Rust website: [https://www.rust-lang.org/](https://www.rust-lang.org/)
2.  Clone this repository: `git clone [repository_url]`
3.  Navigate to the project directory: `cd [project_directory]`
4.  Run the program: `./your_program.sh`