mod parser;
mod tests;
mod types;
mod compiler;
use crate::parser::*;
use crate::compiler::*;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut comp_que = vec!["test.risp".to_owned()];
    let mut i =0; 
    loop{
        let tprg = std::fs::read_to_string(&comp_que[i]).expect("testing expect");
        let prg = program_to_ast(&tprg,&mut comp_que).expect("testing expect");
        let _ = compile(prg,&comp_que[i]).expect("testing expect");
        i += 1;
        if i>=comp_que.len(){
            break;
        }
    }
    print!("linking...");
    let mut cmd =   std::process::Command::new("gcc");
    for i in &comp_que{
        let name = i[0..i.len()-5].to_owned()+".o";
        print!("{} ",name);
        cmd.arg(name);
    }
    let _ = cmd.output().expect("input should be ok");
    let mut cmd = std::process::Command::new("rm");
    for i in &comp_que{
        let name = i[0..i.len()-5].to_owned()+".o";
        println!("{name}");
        cmd.arg(name);
    }
    let _= cmd.output();
}
