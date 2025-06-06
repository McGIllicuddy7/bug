pub mod compiler;
pub mod parser;
pub mod tokenizer;
fn main() {
    let test_str = std::fs::read_to_string("main.bug").unwrap();
    let t = tokenizer::Tokenizer::new(&test_str);
    let tokens: Vec<tokenizer::Token> = t.clone().collect();
    println!("{:#?}", tokens);
    let v = parser::Parser::parse_tokens(t);
    println!("{:#?}", v);
}
