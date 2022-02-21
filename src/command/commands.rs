mod cmd;
mod cmd_cd;

use std::io::Result;

// 0. 넘겨받은 program의 실행 경로 확인 절차
// 1. Command::new 로 실행
// 1-1. 만약 없으면 built-in에서 핵심 명령어들 중 하나인지 체크, 있으면 그거 실행
// 2. 에러 뿜뿜
// pub fn execute(program: &str, envp: &str) -> io::Result<()> {
//     let command = Cmd

//     match is_valid_path
// }