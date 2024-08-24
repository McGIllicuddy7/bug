mod parser;
mod tests;
mod types;
mod compiler;
mod validation;
use crate::parser::*;
use crate::compiler::*;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut comp_que = vec!["test.risp".to_owned()];
    let mut i =0; 
    loop{
        let tprg = "import builtins.risp;\n".to_owned()+&std::fs::read_to_string(&comp_que[i]).expect("testing expect");
        let name = comp_que[i].to_owned();
        let prg = program_to_ast(&tprg,&mut comp_que, &name).expect("testing expect");
        let _ = compile(prg,&comp_que[i]).expect("testing expect");
        i += 1;
        if i>=comp_que.len(){
            break;
        }
    }
    let _= std::process::Command::new("gcc").arg("-c").arg("-std=c2x").arg("builtins.c").output();
    print!("linking...");
    let mut cmd =   std::process::Command::new("gcc");
    for i in &comp_que{
        let name = "output/".to_owned()+&i[0..i.len()-5]+".o";
        print!("{} ",name);
        cmd.arg(name);
    }
    cmd.arg("builtins.o".to_owned());
    print!("builtins.o");
    let _ = cmd.output().expect("input should be ok");
    print!("\ncleaning up...");
    let mut cmd = std::process::Command::new("rm");
    for i in &comp_que{
        let name = "output".to_owned()+&i[0..i.len()-5]+".o";
        print!("{name} ");
        cmd.arg(name);
    }
    cmd.arg("builtins.o");
    let _= cmd.output().expect("command should work");
}
