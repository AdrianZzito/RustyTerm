use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

pub fn cm_no_output(args: Vec<&str>, command: &str) {
    if args.len() < 1 {
        eprintln!("{}: missing filename o directory", command);
    } else {
        let _output = Command::new(command).args(&args[1..]).status().unwrap();
        /*
        if !output.success() {
            eprintln!("{}: command failed", command);
        }
        */
    }
}
