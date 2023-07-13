use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

pub fn cd(args: Vec<&str>) {
    if args.len() < 2 {
        eprintln!("cd: missing directory")
    } else {
        let directory = args[1];
        if let Err(err) = env::set_current_dir(directory) {
            eprintln!("cd: {err}");
        }
    }
}
