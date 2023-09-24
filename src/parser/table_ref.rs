use nom::branch::alt;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::Parser;

use super::token::*;
use crate::ast::table_ref::{JoinCondition, JoinOp, TableName, TableRef};
use crate::parser::common::{ident, match_token};
use crate::parser::expr::expr;
use crate::parser::statement::select_stmt;
use crate::parser::{IResult, Input};

pub fn table_ref(i: Input) -> IResult<TableRef> {
    alt((
        // join,
        subquery, base_table,
    ))(i)
}

fn base_table(i: Input) -> IResult<TableRef> {
    tuple((table_name, opt(table_alias)))(i)
        .map(|(i, (name, alias))| (i, TableRef::BaseTable { name, alias }))
}

fn subquery(i: Input) -> IResult<TableRef> {
    tuple((
        match_token(LParen),
        select_stmt,
        match_token(RParen),
        opt(table_alias),
    ))(i)
    .map(|(i, (_, subquery, _, alias))| {
        (
            i,
            TableRef::Subquery {
                subquery: Box::new(subquery),
                alias,
            },
        )
    })
}

fn join(i: Input) -> IResult<TableRef> {
    tuple((table_ref, join_operator, table_ref, opt(join_condition)))(i).map(
        |(i, (left, op, right, condition))| {
            (
                i,
                TableRef::Join {
                    op,
                    condition: condition.unwrap_or(JoinCondition::None),
                    left: Box::new(left),
                    right: Box::new(right),
                },
            )
        },
    )
}

fn join_condition(i: Input) -> IResult<JoinCondition> {
    alt((tuple((match_token(ON), expr)).map(|(_, expr)| JoinCondition::On(Box::new(expr))),))(i)
}

fn join_operator(i: Input) -> IResult<JoinOp> {
    alt((
        match_token(JOIN).map(|_| JoinOp::Inner),
        tuple((match_token(INNER), match_token(JOIN))).map(|(_, _)| JoinOp::Inner),
        tuple((match_token(LEFT), match_token(JOIN))).map(|(_, _)| JoinOp::LeftOuter),
        tuple((match_token(LEFT), match_token(OUTER), match_token(JOIN)))
            .map(|(_, _, _)| JoinOp::LeftOuter),
        tuple((match_token(RIGHT), match_token(JOIN))).map(|(_, _)| JoinOp::RightOuter),
        tuple((match_token(RIGHT), match_token(OUTER), match_token(JOIN)))
            .map(|(_, _, _)| JoinOp::LeftOuter),
        tuple((match_token(FULL), match_token(JOIN))).map(|(_, _)| JoinOp::FullOuter),
        tuple((match_token(FULL), match_token(OUTER), match_token(JOIN)))
            .map(|(_, _, _)| JoinOp::FullOuter),
    ))(i)
}

fn table_name(i: Input) -> IResult<TableName> {
    alt((
        tuple((ident, match_token(Dot), ident)).map(|(database, _, table)| TableName {
            database: Some(database),
            table,
        }),
        ident.map(|table| TableName {
            database: None,
            table,
        }),
    ))(i)
}

fn table_alias(i: Input) -> IResult<crate::ast::Ident> {
    tuple((match_token(AS), ident))(i).map(|(i, (_, alias))| (i, alias))
}

#[cfg(test)]
mod tests {
    // #[test]
    // pub fn test_join() {
    //     use crate::parser::tokenize_sql;
    //
    //     let tokens = tokenize_sql("t1 join t2 on t1.a = t2.a");
    //     let result = super::join(&tokens);
    //     println!("{:?}", result);
    //     assert!(result.is_ok());
    //     assert_eq!(
    //         format!("{}", result.unwrap().1),
    //         "SELECT *, t1.a, c AS d FROM t1"
    //     );
    // }
}
