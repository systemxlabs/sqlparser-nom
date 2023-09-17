use sqlparser_lalrpop::sql;

#[test]
pub fn parse_select_statement() {
    let result = sql::SetExprParser::new().parse("select a from t");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());

    let result = sql::SetExprParser::new().parse("select * from t where a > ((1 + 2) * 3)");
    assert!(result.is_ok());
    println!("result: {:?}", result.unwrap());
}
