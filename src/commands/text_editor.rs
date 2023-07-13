use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

pub fn text_editor(args: Vec<&str>, command: &str) {
    if args.len() < 2 {
        eprintln!("{}: missing filename", command);
    } else {
        let editor = env::var("EDITOR").unwrap_or_else(|_| command.to_string());
        let filename = args[1];
        let output = Command::new(&editor).arg(&filename).status().unwrap();
        if !output.success() {
            eprintln!("{}: command failed", command);
        }
    }
}
