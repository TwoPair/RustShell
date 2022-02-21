#[warn(non_snake_case)]
mod prompt;
mod command;

use std::env;
use std::io::stdin;
use std::rc::Rc;
use std::cell::RefCell;
use std::process::{Child, Stdio}; // ! "Command" will not use

use command::builtin::BuiltInList as BL;
use command::cmd::CmdPart         as CMDPART;
use command::cmd_cd::CmdChangeDirectory as CD;

/**
 * TODO: 절대/상대 경로상의 명령어 뒤져서 안나오면 다음으로 파일/디렉터리 찾는다
 * TODO: 그럼 빌트인 커맨드는 어떡하지??? -> 명령어 위치인지 인자 위치인지 검사하면 된다! 
 */

fn main() {
    // init process
    let builtin_list = BL {
        blist: vec![
            Rc::new(RefCell::new(
                CD { name: String::from("cd"), }
            )),
        ],
    };
    
    loop {
        // print prompt
        prompt::prompt2();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let cp = CMDPART::new(&mut input);

        builtin_list.execute_cmd(cp.command, &cp.args);

        // // TODO: 1. 절대 경로 명령어 실행 / 2. 상대 경로 명령어 실행
        // Command::new(cmd)
        //         .spawn()
        //         .unwrap();
    }
}

#[cfg(test)]
fn path_print() {
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