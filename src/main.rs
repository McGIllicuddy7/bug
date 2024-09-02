mod tests;
mod types;
mod frontend;
mod ir;
mod compiler;
use crate::frontend::*;
use crate::compiler::*;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut comp_que = vec!["test.bug".to_owned()];
    let mut i =0; 
    loop{
        let tprg = "import builtins.bug;\n".to_owned()+&std::fs::read_to_string(&comp_que[i]).expect("testing expect");
        let name = comp_que[i].to_owned();
        let prg = program_to_ast(&tprg,&mut comp_que, &name).expect("testing expect");
        let _ = compile(prg,&comp_que[i]).expect("testing expect");
        i += 1;
        if i>=comp_que.len(){
            break;
        }
    }
    let t= std::process::Command::new("gcc").arg("-c").arg("-std=c2x").arg("builtins.c").output().expect("should work");
    println!("\n{}",String::from_utf8(t.stderr).expect("should be ut8"));
    print!("linking...");
    let mut cmd =   std::process::Command::new("gcc");
    for i in &comp_que{
        let name = "".to_owned()+&i[0..i.len()-4]+".o";
        print!("{} ",name);
        cmd.arg(name);
    }
    let t = cmd.output().expect("input should be ok");
    println!("\n{}",String::from_utf8(t.stderr).expect("should be ut8"));
    print!("\ncleaning up...");
    let mut cmd = std::process::Command::new("rm");
    for i in &comp_que{
        let name = "".to_owned()+&i[0..i.len()-5]+".o";
        print!("{name} ");
        cmd.arg(name);
    }
    cmd.arg("builtins.o");
    let _= cmd.output().expect("command should work");
}
