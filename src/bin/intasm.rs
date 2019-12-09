#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP

#[test]
fn calculator() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());
}

fn main() {
}
