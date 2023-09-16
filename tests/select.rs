use sqlparser_lalrpop::sql;

#[test]
pub fn parse_select_statement() {
    // let result = sql::UnaryOpExprParser::new().parse("+1.00");
    // assert!(result.is_ok());
    // println!("result: {:?}", result.unwrap());

    let result = sql::AddSubBinaryOpExprParser::new().parse("1 + 2 * 3");
    assert!(result.is_ok());
    println!("result: {:?}", result.unwrap());
}