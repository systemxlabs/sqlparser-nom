use sqlparser_nom::ast::{Expr, Ident};

#[test]
pub fn test_column_ref() {
    use sqlparser_nom::parser::expr::column_ref;
    use sqlparser_nom::parser::tokenize_sql;

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
