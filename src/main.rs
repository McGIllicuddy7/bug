pub mod c;
pub mod compiler;
pub mod interpreter;
pub mod lisp;
fn main() {
    let s = std::fs::read_to_string("main.lisp").unwrap();
    let prog = lisp::parse_string(&s).unwrap();
    //    println!("{prog:#?}");
    let t = compiler::compile(prog);
    //   println!("{t:#?}");
    if let Some(k) = t {
        let p = c::compile_to_c(&k);
        let _ = std::fs::write("main.c", &p);
    }
}
