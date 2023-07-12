use std::io::{self, Write};
use std::process::{exit, Command};
use std::{env, path};

pub fn pwd() -> std::io::Result<()> {
    let path = env::current_dir()?;
    println!("The current directory is {}", path.display());
    Ok(())
}

// fn main() -> std::io::Result<()> {
//    let path = env::current_dir()?;
//    println!("The current directory is {}", path.display());
//    Ok(())
//}
