//use crate::parser::link;

pub mod jit;
pub mod mach;
pub mod parser;
fn main() {
    let p = parser::parse_to_program(
        std::fs::read_to_string("main.beam").unwrap(),
        "main.beam".into(),
    )
    .unwrap();
    println!("{:#?}", p);
    let p2 = parser::parse_to_program(
        std::fs::read_to_string("std.beam").unwrap(),
        "std.beam".into(),
    )
    .unwrap();
    let mut f = parser::link(&[p, p2]);
    let mut count = 0;
    let start = std::time::Instant::now();
    while !f.done {
        count += 1;
        f.update().unwrap();
    }
    let time = std::time::Instant::now() - start;
    f.heap.debug();
    println!(
        "took:{:#?} instructions and {:#?}, for {} instructions per second",
        count,
        time,
        count as f64 / (time.as_secs_f64())
    );
}
