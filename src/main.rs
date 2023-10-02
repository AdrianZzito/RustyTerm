#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

// Access to the files containing the command functions
mod commands {
    pub mod cd;
    pub mod cm_no_output; // file_creation | rm
    pub mod cm_w_output; // ls | catsfdkjnfdjknf
    pub mod pwd;
    pub mod text_editor;
}

fn main() -> Result<()> {
    // Input line creation
    let mut rl = DefaultEditor::new()?;

    // File history logger
    #[cfg(feature = "with-file-history")]
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }

    loop {
        // Casting path as a String to it's usage on the prompt
        let dir = stringed_curr_dir();
        let path = dir.display().to_string() + ":>> ";

        let readline = rl.readline(&path);

        match readline {
            Ok(line) => {
                // Entry addition to the command history
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
                    "nano" | "vim" => {
                        commands::text_editor::text_editor(args, command);
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

    // Saving file history
    #[cfg(feature = "with-file-history")]
    rl.save_history("history.txt");
    Ok(())
}

fn stringed_curr_dir() -> path::PathBuf {
    let path = env::current_dir();
    path.expect("path")
}
