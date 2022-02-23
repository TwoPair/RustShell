use std::{
    io,
    env,
    path::Path,
    iter::Iterator,
    str::SplitWhitespace
};
use super::cmd::Cmd;

////////////////////////////////////////////////////////////////////////////////
// Structures
////////////////////////////////////////////////////////////////////////////////

pub struct CmdChangeDirectory {
    pub name: String,
}

////////////////////////////////////////////////////////////////////////////////
// Common trait implementations for CmdChangeDirectory
////////////////////////////////////////////////////////////////////////////////

impl Cmd for CmdChangeDirectory {
    type Error = io::Error;

    fn execute(&self, args: &SplitWhitespace) {
        let tmp = args.clone();     // peekable() takes ownership... ;(
        let new_dir = tmp.peekable().peek().map_or("/", |&dir| dir);
        let root = Path::new(new_dir);

        let cmd = self.get_cmd_name();
        match env::set_current_dir(&root) {
            // TODO: Ok(_) => {not printing, but logging}
            Ok(_) => println!("{}: built-in successfully called", cmd),
            Err(e) => self.error_handling(e),
        }
    }
    
    fn error_handling(&self, _err: Self::Error) {
        // ex) bash: cd: no such file or directory
        let cmd = self.get_cmd_name();
        // TODO: classify errors below using err value
        println!("{}: no such file or directory", cmd);
    }

    fn get_cmd_name(&self) -> &str {
        self.name.as_ref()
    }
}
