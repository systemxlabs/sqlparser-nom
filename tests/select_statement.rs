use sqlparser_lalrpop::sql;

#[test]
pub fn parse_select_statement() {
    let result = sql::SelectStatementParser::new().parse("select * from t");
    assert!(result.is_ok());
    println!("result: {}", result.unwrap());

    let result = sql::SelectStatementParser::new().parse(
        "
select a, t.b, c 
from t 
where a > ((1 + 2) * 3) and b < c 
group by a, c 
order by a, b desc 
limit 1, 2
",
    );
    assert!(result.is_ok());
    println!("result: {:#?}", result.unwrap());
}
