pub mod arm;
pub mod laigo;
pub mod x86;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("main.lg")?;
    let p = laigo::parse_to_unit(&s)?;
    println!("{:#?}", p);
    arm::compile(p, "main.s");
    Ok(())
}
