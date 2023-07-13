use std::fs::File;
use std::io::{self, stdout, Write};
use std::os::unix::io::{FromRawFd, IntoRawFd};
use std::process::{exit, Command};
use std::{env, path};

pub fn pipes(command: Vec<&str>) {
    let mut previous_output: Option<File> = None;

    for (i, arg) in command.iter().enumerate() {
        let redirect_output = if i < arg.len() - 1 { true } else { false };

        let redirect_input = if let Some(prev_output) = &previous_output {
            prev_output.into_raw_fd()
        } else {
            0
        };

        let redirect_output_fd = if redirect_output {
            let file = File::create("output.txt").unwrap();
            file.into_raw_fd()
        } else {
            1
        };

        let cmd_args: Vec<&str> = arg.split(' ').collect();
        let cmd = cmd_args[0];

        let output = Command::new(cmd)
            .args(&cmd_args[1..])
            .stdin(unsafe { FromRawFd::from_raw_fd(redirect_input) })
            .stdout(unsafe { FromRawFd::from_raw_fd(redirect_output_fd) })
            .output()
            .unwrap();

        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        if let Some(prev_output) = previous_output {
            unsafe { libc::close(prev_output.into_raw_fd()) };
        }

        /*
        if redirect_output {
            previous_output = Some(unsafe { File::from_raw_fd(output.stdout.as_raw_fd()) });
        }
        */
    }

    if let Some(prev_output) = previous_output {
        unsafe { libc::close(prev_output.into_raw_fd()) };
    }
}
