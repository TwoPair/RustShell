use std::io::{stdout, Write};

pub fn prompt2() {
    // need to explicitly flush this to ensure it prints before read_line
    // TODO: tuning prompt fit to various settings
    print!("> ");
    stdout().flush().unwrap();
}