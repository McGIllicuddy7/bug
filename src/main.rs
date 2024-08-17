mod parser;
mod tests;
mod types;
mod compiler;
use crate::parser::*;
use crate::compiler::*;
fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    let tprg = std::fs::read_to_string("test.risp").expect("testing expect");
    let prg = program_to_ast(&tprg).expect("testing expect");
    println!("{:#?}", prg);
    let _ = compile(prg,"test.risp").expect("testing expect");
}
