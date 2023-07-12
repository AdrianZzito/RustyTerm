use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

pub fn file_creation(args: Vec<&str>, command: &str) {
    if args.len() < 2 {
        eprintln!("{}: missing filename o directory", command);
    } else {
        let output = Command::new(command).args(&args[1..]).status().unwrap();
        if !output.success() {
            eprintln!("{}: command failed", command);
        }
    }
}
