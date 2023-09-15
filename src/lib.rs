use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub calculator); // synthesized by LALRPOP

#[test]
fn calculator1() {
    assert!(calculator::TermParser::new().parse("22").is_ok());
    assert!(calculator::TermParser::new().parse("(22)").is_ok());
    assert!(calculator::TermParser::new().parse("((((22))))").is_ok());
    assert!(calculator::TermParser::new().parse("((22)").is_err());

    assert!(calculator::ExprParser::new().parse("1 + 2 * 3").is_ok());
    println!("{:?}", calculator::ExprParser::new().parse("1 + 2 * 3").unwrap());
    println!("{:?}", calculator::ExprParser::new().parse("1 * 2 + 3").unwrap());
}
