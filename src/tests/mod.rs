use crate::*;
#[allow(unused)]
const PRG1: &str = "{hello world}";
#[allow(unused)]
const PRG2: &str = "{{hello world}}";
#[test]
fn test_end_scope() {
    let tokens = tokenize(PRG1);
    eprintln!("tokens:{:#?}", tokens);
    let end = calc_close_scope(&tokens, 0).expect("prg1 should work");
    assert!(end == 3);
    let tokens = tokenize(PRG2);
    let end = calc_close_scope(&tokens, 0).expect("prg2 should work");
    assert!(end == 5);
}
