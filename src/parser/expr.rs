use nom::Parser;
use nom::{branch::alt, sequence::tuple};

use crate::ast::expr::Expr;
use crate::parser::token::*;

use super::{
    common::{ident, match_token},
    IResult, Input,
};

pub fn column_ref(i: Input) -> IResult<Expr> {
    alt((
        tuple((ident, match_token(Dot), ident, match_token(Dot), ident)).map(
            |(database, _, table, _, column)| Expr::ColumnRef {
                database: Some(database),
                table: Some(table),
                column,
            },
        ),
        tuple((ident, match_token(Dot), ident)).map(|(table, _, column)| Expr::ColumnRef {
            database: None,
            table: Some(table),
            column,
        }),
        ident.map(|column| Expr::ColumnRef {
            database: None,
            table: None,
            column,
        }),
    ))(i)
}

mod tests {
    #[test]
    pub fn test_column_ref() {
        use super::*;
        use crate::ast::Ident;
        use crate::parser::expr::column_ref;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("t1.b");
        let column_ref = column_ref(&tokens);
        println!("{:?}", column_ref);
        assert!(column_ref.is_ok());
        let column_ref = column_ref.unwrap();
        assert_eq!(column_ref.0, vec![]);
        match column_ref.1 {
            Expr::ColumnRef {
                database,
                table,
                column,
            } => {
                assert_eq!(database, None);
                assert_eq!(
                    table,
                    Some(Ident {
                        value: "t1".to_string()
                    })
                );
                assert_eq!(
                    column,
                    Ident {
                        value: "b".to_string()
                    }
                );
            }
            _ => panic!("should be column ref"),
        }
    }
}
