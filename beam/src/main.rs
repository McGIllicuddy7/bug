use crate::fast::IntermediateRt;

pub mod fast;
pub mod heap;
pub mod mach;
pub mod parser;
pub struct Timer {
    start: std::time::Instant,
}
impl Timer {
    pub fn new() -> Self {
        Self {
            start: std::time::Instant::now(),
        }
    }
}
impl Drop for Timer {
    fn drop(&mut self) {
        let dr = std::time::Instant::now()
            .checked_duration_since(self.start)
            .unwrap();
        println!("took:{:#?}", dr);
    }
}
pub fn compile() -> IntermediateRt {
    let s = include_str!("../main.beam");
    let p = parser::parse_to_program(s.to_string(), "main.beam".into()).unwrap();
    println!("{:#?}", p);
    let std = include_str!("../std.beam");
    let p2 = parser::parse_to_program(std.to_string(), "std.beam".into()).unwrap();
    let f = fast::compile_mach_to_ir(&[p, p2]);
    f
}
pub fn fast() {
    let run = true;
    let comp = true;
    if comp {
        let rt = compile();
        let v = rmp_serde::to_vec(&rt).unwrap();
        std::fs::write("test.bin", v).unwrap();
    }
    if run {
        let s = std::fs::read("test.bin").unwrap();
        let rt: IntermediateRt = rmp_serde::from_slice(&s).unwrap();
        let mut f = fast::rt_from_intermediate_rt(rt);
        while !f.halted {
            f.step().unwrap();
        }
    }
}
fn main() {
    let s = include_str!("../main.beam");
    let p = parser::parse_to_program(s.to_string(), "main.beam".into()).unwrap();
    println!("{:#?}", p);
    let std = include_str!("../std.beam");
    let p2 = parser::parse_to_program(std.to_string(), "std.beam".into()).unwrap();
    let mut machine = parser::link(&[p, p2]);
    while !machine.done {
        machine.update().unwrap();
    }
}
