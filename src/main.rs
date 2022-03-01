#[allow(non_snake_case)]
mod prompt;
mod command;

use std::io::stdin;
use std::rc::Rc;
use std::cell::RefCell;

use command::builtin::BuiltInList as BL;
use command::cmd::CmdPart         as CMDPART;
use command::cmd_cd::CmdChangeDirectory  as CD;
use command::cmd_pwd::CmdPwd             as PWD;
use command::cmd_chat::CmdChat           as CHAT;
use command::cmd_fileshare::CmdFileShare as FILESHARE;

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
    ($bl:ident, {$([$s:ident, $name:tt]),*}) => {
        $bl {
            blist: multiaccessvec_constructor!($($s, $name),*),
        };
    };
}

#[tokio::main]
async fn main() {
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
    let builtin_list = builtin_constructor!(BL,
        {
            [CD, "cd"],
            [PWD, "pwd"],
            [CHAT, "chat"],
            [FILESHARE, "fileshare"]
        }
    );

    // TODO: cmd_chat용 json 파일 초기화 방법론 고려하기
    use std::fs::File;
    use command::consts::CHAT_USER_DB;
    // create() destroy the old content if the file already existed.
    File::create(CHAT_USER_DB).unwrap();
    
    loop {
        // print prompt
        prompt::prompt2();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        if input == "exit" {
            // TODO: check if there were any running command or process
            return;
        } else {
            let cp = CMDPART::new(&mut input);
            builtin_list.execute_cmd(cp.command, &cp.args);
        }
        
        // Command::new(cmd)
        //         .spawn()
        //         .unwrap();
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn print_path() {
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
}
