use nom::branch::alt;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::Parser;

use crate::ast::statement::OrderByExpr;
use crate::ast::{expr::Expr, statement::SelectStatement};
use crate::parser::token::*;

use super::common::comma_separated_list1;
use super::{common::match_token, expr::expr, set_expr::select_set_expr, IResult, Input};

pub fn select_stmt(i: Input) -> IResult<SelectStatement> {
    tuple((
        select_set_expr,
        opt(order_by_clause),
        opt(limit_offset_clause),
    ))(i)
    .map(|(i, (select, order_by, limitoffset))| {
        let (limit, offset) = limitoffset.unwrap_or((None, None));
        (
            i,
            SelectStatement {
                body: select,
                order_by: order_by.unwrap_or(vec![]),
                limit,
                offset,
            },
        )
    })
}

fn order_by_clause(i: Input) -> IResult<Vec<OrderByExpr>> {
    tuple((
        match_token(ORDER),
        match_token(BY),
        comma_separated_list1(order_by_expr),
    ))(i)
    .map(|(i, (_, _, order_by_list))| (i, order_by_list))
}
fn order_by_expr(i: Input) -> IResult<OrderByExpr> {
    alt((
        tuple((expr, match_token(ASC))).map(|(expr, _)| OrderByExpr {
            expr,
            asc: Some(true),
        }),
        tuple((expr, match_token(DESC))).map(|(expr, _)| OrderByExpr {
            expr,
            asc: Some(false),
        }),
        expr.map(|expr| OrderByExpr { expr, asc: None }),
    ))(i)
}

fn limit_offset_clause(i: Input) -> IResult<(Option<Expr>, Option<Expr>)> {
    alt((
        tuple((match_token(LIMIT), expr, match_token(OFFSET), expr))
            .map(|(_, limit, _, offset)| (Some(limit), Some(offset))),
        tuple((match_token(LIMIT), expr, match_token(Comma), expr))
            .map(|(_, limit, _, offset)| (Some(limit), Some(offset))),
        tuple((match_token(LIMIT), expr)).map(|(_, limit)| (Some(limit), None)),
        tuple((match_token(OFFSET), expr)).map(|(_, offset)| (None, Some(offset))),
    ))(i)
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_select_stmt() {
        use super::select_stmt;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("select a, t1.b from t1 order by a, b desc limit 1, 2");
        let result = select_stmt(&tokens);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0, vec![]);
        println!("select_stmt: {}", result.1);
    }
}
