pub mod interpreter;
pub mod miku;
pub mod tuci;
fn main() {
    let str = std::fs::read_to_string("test.miku").unwrap();
    let state = miku::ParserState::parse_to_program(&str);
    if let Ok(state) = state {
        println!("{:#?}", state);
    } else if let Err(state) = state {
        println!("{}", state);
    }
}
