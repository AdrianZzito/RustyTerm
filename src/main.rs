#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

mod commands {
    pub mod cat;
    pub mod cd;
    pub mod file_creation;
    pub mod ls;
    pub mod pwd;
    pub mod rm;
}

fn main() {
    let mut _command_history: &[&str];

    loop {
        let dir = stringed_curr_dir();

        print!("{}:>>", dir.display());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let input = input.trim();

        let args: Vec<&str> = input.split(' ').collect();
        if args.is_empty() {
            continue;
        }

        let command = args[0];

        match command {
            "ls" => {
                commands::ls::ls(args);
            }
            "cd" => {
                commands::cd::cd(args);
            }
            "cat" => {
                commands::cat::cat(args);
            }
            "mkdir" | "touch" => {
                commands::file_creation::file_creation(args, command);
            }
            "pwd" => {
                commands::pwd::pwd();
            }
            "rm" | "rmdir" => {
                commands::rm::rm(args, command);
            }
            "exit" => {
                print!("Closing terminal: exit code 0");
                exit(0);
            }
            _ => {
                eprintln!("Command not found: {command}");
            }
        }
        // command_history.append(command);
        // statement to add the command introduced to the command_history array so when you hit the arrow up button you will type that command
    }
}

fn stringed_curr_dir() -> path::PathBuf {
    let path = env::current_dir();
    path.expect("path")
}
