use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

pub fn cm_w_output(args: Vec<&str>, command: &str) {
    let output = Command::new(command).args(&args[1..]).output().unwrap();
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
