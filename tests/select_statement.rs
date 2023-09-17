use sqlparser_lalrpop::sql;

#[test]
pub fn parse_select_statement() {
    let result = sql::StatementParser::new().parse("select * from t");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());

    let result = sql::StatementParser::new()
        .parse("select a, b, c from t where a > ((1 + 2) * 3) limit 1, 2");
    assert!(result.is_ok());
    println!("result: {:?}", result.unwrap());
}
