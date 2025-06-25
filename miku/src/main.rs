pub mod interpreter;
pub mod miku;
pub mod tuci;
pub mod utils;
pub fn compile() {
    let str = std::fs::read_to_string("test.miku").unwrap();
    let state = miku::ParserState::parse_to_program(&str);

    if let Ok(state) = state {
        let state = miku::MikuObject::link(&[state]).unwrap();
        println!("{:#?}", state);
        let prog = tuci::tuci(state);
        std::fs::write("test.c", prog).unwrap();
    } else if let Err(state) = state {
        println!("{}", state);
    }
}
fn main() {
    println!("{:#?}", utils::extract_string_literals("\"hello \"there"));
}
