mod parser;
mod tests;
mod types;
mod compiler;
use crate::parser::*;
use crate::compiler::*;
use std::os::unix::process::CommandExt;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut comp_que = vec!["test.risp".to_owned()];
    let mut i =0; 
    while i<comp_que.len(){
        let tprg = std::io::read_to_string(&comp_que[i]);
        let prg = program_to_ast(&tprg,&mut comp_que).expect("testing expect");
        println!("{:#?}", prg);
        let _ = compile(prg,comp_que[i]).expect("testing expect");
    }
    let mut cmd =   std::process::Command::new("ld");

    for i in comp_que{
        let name = i[0..i.len()-5]+".o";
        cmd.arg(name);
    }
    cmd.exec();
}
