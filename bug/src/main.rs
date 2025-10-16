pub mod bug;
pub mod compiler;
pub mod parser;
pub mod tokens;
pub mod utils;
pub fn repl() {
    loop {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        //let t = line.strip_suffix("\n").unwrap();
        if &line == "exit\n" {
            return;
        }
        let mut tokens = tokens::TokenStream::new(&line, "stdin");
        let toks = tokens.collect();
        println!("{:#?}", toks);
        let exp = parser::parse_expression(&toks).unwrap();
        println!("{:#?}", exp);
    }
}
pub fn parse(filename: &str) {
    let s = std::fs::read_to_string(filename).unwrap();
    let token = tokens::TokenStream::new(&s, filename).collect();
    let prg = bug::parse_program(token).unwrap();
    println!("{:#?}", prg);
}
fn main() {
    parse("main.bug");
    //repl();
}
