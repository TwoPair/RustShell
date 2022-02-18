#[warn(non_snake_case)]

use std::env;
use std::io::{stdin, stdout, Write};
use std::path::Path;
use std::process::{Child, Command, Stdio}; // ! will not use

fn main() {
    loop {
        // need to explicitly flush this to ensure it prints before read_line
        // TODO: tuning prompt fit to various settings
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        // read_line leaves a trailing 
        let cmd = input.trim();

        // ! Deprecated at next commit
        Command::new(cmd)
                .spawn()
                .unwrap();
    }
}
