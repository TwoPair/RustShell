use std::{
    io,
    env,
    str::SplitWhitespace
};
use super::cmd::Cmd;

////////////////////////////////////////////////////////////////////////////////
// Structures
////////////////////////////////////////////////////////////////////////////////

pub struct CmdFileShare {
    pub name: String,
}

////////////////////////////////////////////////////////////////////////////////
// Common trait implementations for CmdFileShare
////////////////////////////////////////////////////////////////////////////////

impl Cmd for CmdFileShare {
    type Error = io::Error;

    fn execute(&self, _args: &SplitWhitespace) {
        
    }
    
    fn error_handling(&self, _err: Self::Error) {
        let cmd = self.get_cmd_name();
        // TODO: classify errors below using err value
        println!("{}: {}", cmd, "nothing");
    }

    fn get_cmd_name(&self) -> &str {
        self.name.as_ref()
    }
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl CmdFileShare {
    
}
