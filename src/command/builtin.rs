use std::str::SplitWhitespace;
use std::{
    rc::Rc,
    cell::RefCell
};
use super::cmd::Cmd;

////////////////////////////////////////////////////////////////////////////////
// Type Aliases
////////////////////////////////////////////////////////////////////////////////

type Inner<T> = Rc<RefCell<T>>;
type MultiAccessVec<T> = Vec<Inner<T>>;

////////////////////////////////////////////////////////////////////////////////
// Structures
////////////////////////////////////////////////////////////////////////////////

// [struct BuiltInList<T: Cmd>]
// - Manage the Built-in command list
// * Must be initialized in the **main.rs**
// * before using inner functions.
pub struct BuiltInList<T>
    where T: Cmd,
{
    pub blist: MultiAccessVec<T>,
}

////////////////////////////////////////////////////////////////////////////////
// Inherent methods
////////////////////////////////////////////////////////////////////////////////

impl<T> BuiltInList<T>
    where T: Cmd,
{
    pub fn get_builtin_list(&self) -> &MultiAccessVec<T> {
        &self.blist
    }

    pub fn execute_cmd(&self, cmd: &str, args: &SplitWhitespace) {
        // find the target cmd & execute it
        for b in self.blist.iter() {
            let b = b.borrow();
            if cmd == b.get_cmd_name() {
                // TODO: logging system
                match b.execute(args) {
                    Ok(_) => println!("[{}] built-in successfully called", cmd),
                    Err(e) => println!("[{}] {}", cmd, e),
                }
                break;
            }
        }
    }
}