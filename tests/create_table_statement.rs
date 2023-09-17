use sqlparser_lalrpop::sql::StatementParser;

#[test]
fn parse_create_table_statement() {
    let result = StatementParser::new().parse(
        "
        create table student (
            id integer PRIMARY KEY,
            age smallint(2) CONSTRAINT age_not_null not null
        )
    ",
    );
    assert!(result.is_ok());
    println!("result: {:?}", result.unwrap());
}
