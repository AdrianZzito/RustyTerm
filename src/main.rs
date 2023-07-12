#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

mod commands {
    pub mod cd;
    pub mod cm_no_output; // file_creation | rm
    pub mod cm_w_output; // ls | cat
    pub mod pwd;
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
                commands::cm_w_output::cm_w_output(args, command);
            }
            "cd" => {
                commands::cd::cd(args);
            }
            "cat" => {
                commands::cm_w_output::cm_w_output(args, command);
            }
            "mkdir" | "touch" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "pwd" => {
                commands::pwd::pwd();
            }
            "rm" | "rmdir" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "sudo" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "apt" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "cp" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "mv" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "man" => {
                commands::cm_w_output::cm_w_output(args, command);
            }
            "chmod" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "unzip" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "echo" => {
                commands::cm_w_output::cm_w_output(args, command);
            }
            "ps" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "kill" => {
                commands::cm_no_output::cm_no_output(args, command);
            }
            "tail" => {
                commands::cm_w_output::cm_w_output(args, command);
            }
            "head" => {
                commands::cm_w_output::cm_w_output(args, command);
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
