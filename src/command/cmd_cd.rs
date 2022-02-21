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
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl Cmd for CmdChangeDirectory {
    fn execute(&self, args: &SplitWhitespace) -> io::Result<()> {
        let tmp = args.clone();     // peekable() takes ownership... ;(
        let new_dir = tmp.peekable().peek().map_or("/", |&dir| dir);
        let root = Path::new(new_dir);

        let err = env::set_current_dir(&root);
        self.error_handling(err)
    }

    fn get_cmd_name(&self) -> &str {
        self.name.as_ref()
    }

    fn error_handling(&self, err: io::Result<()>) -> io::Result<()> {
        // TODO: print error script
        // ex) bash: cd: no such file or directory
        err
    }
}