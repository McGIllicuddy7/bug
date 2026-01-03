//use crate::parser::link;

pub mod mach;
pub mod parser;
//2,358,501.260279928 instructions per second 
//with release 5,845,995.525498529 instructions per second
//more non release2,843,604.739336493 instructions per second
//with release 5,949,076.505116206
fn main() {
    let p = parser::parse_to_program(
        std::fs::read_to_string("main.beam").unwrap(),
        "main.beam".into(),
    )
    .unwrap();
    println!("{:#?}", p);
    let mut f = parser::link(&[p]);
    let mut count =0;
    let start = std::time::Instant::now();
    while !f.done {
        count+=1;
        f.update().unwrap();
    }    
    let time = std::time::Instant::now()-start;
    f.heap.debug();
    println!("took:{:#?} instructions and {:#?}", count, time);

}
