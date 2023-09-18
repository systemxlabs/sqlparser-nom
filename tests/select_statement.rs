use sqlparser_lalrpop::sql;

#[test]
pub fn parse_select_statement() {
    let result = sql::StatementParser::new().parse("select * from t");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());

    let result = sql::StatementParser::new().parse(
        "select a, t.b, c from t where a > ((1 + 2) * 3) and b < c order by a, b desc limit 1, 2",
    );
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());
}
