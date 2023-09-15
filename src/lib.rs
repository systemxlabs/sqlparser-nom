use lalrpop_util::lalrpop_mod;

pub mod ast;

lalrpop_mod!(pub sql);

#[cfg(test)]
mod tests {
    #[test]
    fn parse_ident() {
        use crate::sql;
        let result = sql::IdentParser::new().parse("t1");
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_column_def() {
        use crate::sql;
        let result = sql::ColumnDefParser::new().parse("id tinyint PRIMARY KEY NOT NULL");
        println!("result: {:?}", result);
        assert!(result.is_ok());

        let result = sql::ColumnDefParser::new().parse("age tinyint CONSTRAINT age_not_null NOT NULL");
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn parse_create_table_statement() {
        use crate::sql;
        let result = sql::CreateTableStatementParser::new().parse("
            create table student (
                id tinyint PRIMARY KEY,
                age tinyint not null
            )
        ");
        println!("result: {:?}", result);
        assert!(result.is_ok());
    }
}
