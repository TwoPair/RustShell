use std::io;
use std::str::SplitWhitespace;
use std::{
    rc::Rc,
    cell::RefCell
};
use super::cmd::Cmd;

type Wrapper = Rc<RefCell<dyn Cmd<Error = io::Error>>>;
type MultiAccessVec = Vec<Wrapper>;

// [struct BuiltInList<T: Cmd>]
// - Manage the Built-in command list
// * Must be initialized in the **main.rs**
// * before using inner functions.
pub struct BuiltInList {
    pub blist: MultiAccessVec,
}

impl BuiltInList {
    #[allow(dead_code)]
    pub fn get_builtin_list(&self) -> &MultiAccessVec {
        &self.blist
    }

    pub fn execute_cmd(&self, cmd: &str, args: &SplitWhitespace) {
        // find the target cmd & execute it
        for b in self.blist.iter() {
            let b = b.borrow();
            if cmd == b.get_cmd_name() {
                b.execute(args);
                break;
            }
        }
    }
}
