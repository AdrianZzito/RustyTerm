#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

mod commands {
    pub mod cd;
    pub mod cm_no_output; // file_creation | rm
    pub mod cm_w_output; // ls | cat
    pub mod pwd;
}

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        let dir = stringed_curr_dir();
        let path = dir.display().to_string() + ":>> ";

        let readline = rl.readline(&path);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                let input = line.trim();

                let args: Vec<&str> = input.split(' ').collect();
                if args.is_empty() {
                    continue;
                }

                let command = args[0];

                match command {
                    "ls" | "cat" | "man" | "echo" | "tail" | "head" => {
                        commands::cm_w_output::cm_w_output(args, command);
                    }
                    "mkdir" | "touch" | "rm" | "rmdir" | "sudo" | "apt" | "cp" | "mv" | "chmod"
                    | "unzip" | "ps" | "kill" => {
                        commands::cm_no_output::cm_no_output(args, command);
                    }
                    "cd" => {
                        commands::cd::cd(args);
                    }
                    "pwd" => {
                        commands::pwd::pwd();
                    }
                    "exit" => {
                        print!("Closing terminal...");
                        exit(0);
                    }
                    _ => {
                        eprintln!("Command not found: {command}");
                    }
                }
            }

            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C: Interrupted");
                break;
            }

            Err(ReadlineError::Eof) => {
                println!("CTRL-D: Exit");
                break;
            }

            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
        // command_history.append(command);
        // statement to add the command introduced to the command_history array so when you hit the arrow up button you will type that command
    }
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}

fn stringed_curr_dir() -> path::PathBuf {
    let path = env::current_dir();
    path.expect("path")
}
