use std::io::{stdout, Write};

pub fn prompt2() {
    // TODO: tuning prompt fit to various settings
    // need to explicitly flush this to ensure it prints before read_line
    print!("> ");
    stdout().flush().unwrap();
}
