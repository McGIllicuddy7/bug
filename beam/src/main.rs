//use crate::parser::link;

pub mod mach;
pub mod parser;
fn main() {
    let p = parser::parse_to_program(
        std::fs::read_to_string("main.beam").unwrap(),
        "main.beam".into(),
    )
    .unwrap();
    println!("{:#?}", p);
    //    let mut f = link(&[p]);
    //   while !f.done {
    //      f.update().unwrap();
    // }
}
