use nom::Parser;
use nom::{branch::alt, sequence::tuple};

use crate::ast::Expr;
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
