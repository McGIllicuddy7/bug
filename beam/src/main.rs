//use crate::parser::link;

pub mod fast;
pub mod mach;
pub mod parser;
pub struct Timer{
    start:std::time::Instant,
}
impl Timer{
    pub fn new()->Self{
        Self{start:std::time::Instant::now()}
    }
}
impl Drop for Timer{
    fn drop(&mut self) {
        let dr = std::time::Instant::now().checked_duration_since(self.start).unwrap();
        println!("took:{:#?}",dr);
    }
}
fn main() {
    let s = include_str!("../main.beam");
    let p = parser::parse_to_program(
        s.to_string(),
        "main.beam".into(),
    )
    .unwrap();
    println!("{:#?}",p);
    let std = include_str!("../std.beam");
    let p2 = parser::parse_to_program(
        std.to_string(),
        "std.beam".into(),
    )
    .unwrap();
    let mut machine = parser::link(&[p.clone(), p2.clone()]);
    let t2 = Timer::new();
    let mut count:usize =0;
    while !machine.done{
       count+=1;
      machine.update().unwrap();
     }
    drop(t2);
    println!("{} instructions", count);
    count = 0;
    let mut f = fast::compile_mach_to_rt(&[p, p2]);
    f.debug_instrs();
    let t = Timer::new();
    while !f.halted{
        count+=1;
        f.step().unwrap();
    }
    println!("{} instructions", count);
    drop(t);


}
