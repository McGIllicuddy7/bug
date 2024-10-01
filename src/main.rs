mod tests;
mod types;
mod frontend;
mod ir;
mod c_comp;
mod asm_comp;
mod gc;
use std::collections::HashSet;
use std::process::exit;

use gc::compile_gc_functions;

use crate::frontend::*;
use crate::c_comp::*;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let mut comp_que = vec!["test.bug".to_owned()];
    let mut i =0; 
    let target = match std::env::consts::OS{
        "linux"=>{
            Target::Linux { arm: false }
        }
        "macos"=>{
            Target::MacOs { arm: false }
        }
        _=>{
            unreachable!();
        }
    }; 
    let to_c_code = false;
    let mut global_used_types = HashSet::new();
    loop{
        let tprg = "import builtins.bug;\n".to_owned()+&std::fs::read_to_string(&comp_que[i]).expect("testing expect");
        let name = comp_que[i].to_owned();
        let prg = match program_to_ast(&tprg,&mut comp_que, &name){
            Some(t)=>{
                t
            }
            None=>{
                exit(1);
            }
        };
        if to_c_code{
            let _ = compile(prg,&comp_que[i],&mut global_used_types).expect("testing expect");
        }
        else{
            let _ = asm_comp::compile_to_asm_x86(prg, &comp_que[i], &mut global_used_types,&target);
        }
        i += 1;
        if i>=comp_que.len(){
            break;
        }
    }
    print!("linking...");
    let mut cmd =   std::process::Command::new("clang");
    for i in &comp_que{
        if i == "builtins.bug"{
            continue;
        }
        let name = "output/".to_owned()+&i[0..i.len()-4];
        print!("{} ",name);
        if !to_c_code{
            cmd.arg(name+".o");
        }else{
            cmd.arg(name+".c");
        }

    }
    compile_gc_functions(&global_used_types, &target);
    cmd.arg("output/gc_functions.c");
    cmd.arg("builtins.c").arg("-std=c2x");
    let t = cmd.output().expect("input should be ok");
    println!("\n{}",String::from_utf8(t.stderr).expect("should be ut8"));
    print!("\ncleaning up...");
    let mut cmd = std::process::Command::new("rm");
    for i in &comp_que{
        if i == "builtins.bug"{
            continue;
        }
        let name = "output/".to_owned()+&i[0..i.len()-4]+".o";
        print!("{name} ");
        cmd.arg(&name);
        cmd.arg(name+"_gc_funcs.o");
    }
    cmd.arg("builtins.o");
    cmd.arg("output/builtins.o");
    let _= cmd.output().expect("command should work");
}
