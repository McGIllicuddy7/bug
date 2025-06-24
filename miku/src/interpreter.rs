pub use crate::miku::MikuInstr;
pub struct MikuVar {}
pub struct Interpreter {
    pub variables: Vec<MikuVar>,
    pub instructions: Vec<MikuInstr>,
}
