pub mod compiler;
pub mod interpreter;
pub mod lisp;
fn main() {
    let prog = lisp::parse_string("(print \"i love toast\")").unwrap();
    println!("{:#?}", prog);
}
