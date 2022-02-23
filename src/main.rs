#[warn(non_snake_case)]
mod prompt;
mod command;

use std::io::stdin;
use std::rc::Rc;
use std::cell::RefCell;
use std::process::{Child, Stdio}; // ! "std::process::Command" will not use

use command::builtin::BuiltInList as BL;
use command::cmd::CmdPart         as CMDPART;
use command::cmd_cd::CmdChangeDirectory as CD;
use command::cmd_pwd::CmdPwd            as PWD;

// TODO: 절대/상대 경로상의 명령어 뒤져서 안나오면 다음으로 파일/디렉터리 찾는다

macro_rules! multiaccessvec_constructor {
    ($($s:ident, $name:tt),*) => {
        vec![
        $(
            Rc::new(RefCell::new(
                $s { name: String::from($name), }
            )),
        )*  // $( somthing )*    <- repeat for length of patterns
        ]
    };
}

macro_rules! builtin_constructor {
    ($bl:ident, $var:ident, {$([$s:ident, $name:tt]),*}) => {
        let $var = $bl {
            blist: multiaccessvec_constructor!($($s, $name),*),
        };
    };
}

fn main() {
    // [init process]
    // Expected the result of macro ->
    // ```
    // let builtin_list = BL {
    //     blist: vec![
    //         Rc::new(RefCell::new(
    //             CD { name: String::from("cd"), }
    //         )),
    //         Rc::new(RefCell::new(
    //             PWD { name: String::from("pwd"), }
    //         )),
    //     ],
    // };
    // ```
    builtin_constructor!(BL, builtin_list,
        {
            [CD, "cd"],
            [PWD, "pwd"]
        }
    );
    
    loop {
        // print prompt
        prompt::prompt2();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let cp = CMDPART::new(&mut input);

        builtin_list.execute_cmd(cp.command, &cp.args);

        // Command::new(cmd)
        //         .spawn()
        //         .unwrap();
    }
}

#[cfg(test)]
fn path_print() {
    use std::env;

    let key = "PATH";
    match env::var_os(key) {
        Some(paths) => {
            for path in env::split_paths(&paths) {
                println!("{}", path.display());
            }
        }
        None => println!("couldn't interpret {}", key),
    }
}
