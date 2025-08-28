pub mod arm;
pub mod laigo;
use std::error::Error;
fn main() -> Result<(), Box<dyn Error>> {
    let s = std::fs::read_to_string("main.lg")?;
    let p = laigo::parse_to_unit(&s)?;
    arm::compile(p, "main.s");
    Ok(())
}
