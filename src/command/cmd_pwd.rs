use std::{
    io,
    env,
    path::PathBuf,
    iter::Iterator,
    str::SplitWhitespace
};
use super::cmd::Cmd;

////////////////////////////////////////////////////////////////////////////////
// Structures
////////////////////////////////////////////////////////////////////////////////

pub struct CmdPwd {
    pub name: String,
}

////////////////////////////////////////////////////////////////////////////////
// Common trait implementations for CmdPwd
////////////////////////////////////////////////////////////////////////////////

impl Cmd for CmdPwd {
    type Error = io::Error;

    fn execute(&self, _args: &SplitWhitespace) {
        match env::current_dir() {
            Ok(path) => println!("{}", path.display()),
            Err(e) => self.error_handling(e),
        }
    }
    
    fn error_handling(&self, err: Self::Error) {
        let cmd = self.get_cmd_name();
        // TODO: classify errors below using err value
        let err_str1 = "Current directory does not exist";
        let err_str2 = "There are insufficient permissions to access the current directory";
        println!("{}: [{}] OR [{}]", cmd, err_str1, err_str2);
    }

    fn get_cmd_name(&self) -> &str {
        self.name.as_ref()
    }
}
