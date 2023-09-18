use sqlparser_lalrpop::sql;

#[test]
pub fn parse_expr() {
    let result = sql::ExprParser::new().parse("4 - (1 + 2) * 3");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());

    let result = sql::ExprParser::new().parse("a + b * c");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());

    let result = sql::ExprParser::new().parse("a > (1 + 2)");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());

    let result = sql::ExprParser::new().parse("a.b");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());
}
