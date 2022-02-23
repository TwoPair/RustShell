use std::iter::Iterator;
use std::str::SplitWhitespace;

////////////////////////////////////////////////////////////////////////////////
// Functions
////////////////////////////////////////////////////////////////////////////////

fn parse_input<'a>(input: &'a mut str) -> (&'a str, SplitWhitespace<'a>) {
    let mut parts = input.trim().split_whitespace();
    let cmd = parts.next().unwrap();
    let args = parts;

    (cmd, args)
}

////////////////////////////////////////////////////////////////////////////////
// Traits
////////////////////////////////////////////////////////////////////////////////

// [Trait: Cmd]
// - Command interface
pub trait Cmd {
    type Error;

    // actual functionality of command
    fn execute(&self, args: &SplitWhitespace);

    // handling err
    // * Success result should remain logs or feedback to caller.
    fn error_handling(&self, err: Self::Error);
    
    // return current command name
    fn get_cmd_name(&self) -> &str;
}

////////////////////////////////////////////////////////////////////////////////
// Structures
////////////////////////////////////////////////////////////////////////////////

// [struct CmdPart<'a>]
// - save parsing of input
// TODO: it will be associated with other CmdPart structs
pub struct CmdPart<'a> {
    pub command: &'a str,
    pub args: SplitWhitespace<'a>,
    prev_cmd: bool,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<'a> CmdPart<'a> {
    pub fn new(input: &'a mut str) -> Self {
        let (cmd, ag) = parse_input(input);
        CmdPart {
            command: cmd,
            args: ag,
            prev_cmd: false,
        }
    }
}
