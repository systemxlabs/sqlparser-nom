use nom::branch::alt;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::Parser;

use crate::ast::statement::{Cte, OrderByExpr, TableAlias, With};
use crate::ast::{expr::Expr, statement::SelectStatement};
use crate::parser::common::ident;
use crate::parser::token::*;

use super::common::comma_separated_list1;
use super::{common::match_token, expr::expr, set_expr::select_set_expr, IResult, Input};

pub fn select_stmt(i: Input) -> IResult<SelectStatement> {
    tuple((
        opt(with_clause),
        select_set_expr,
        opt(order_by_clause),
        opt(limit_offset_clause),
    ))(i)
    .map(|(i, (with, select, order_by, limitoffset))| {
        let (limit, offset) = limitoffset.unwrap_or((None, None));
        (
            i,
            SelectStatement {
                with,
                body: select,
                order_by: order_by.unwrap_or(vec![]),
                limit,
                offset,
            },
        )
    })
}

fn with_clause(i: Input) -> IResult<With> {
    tuple((
        match_token(WITH),
        opt(match_token(RECURSIVE)),
        comma_separated_list1(cte),
    ))(i)
    .map(|(i, (_, recursive, cte_tables))| {
        (
            i,
            With {
                recursive: recursive.is_some(),
                cte_tables,
            },
        )
    })
}

fn cte(i: Input) -> IResult<Cte> {
    tuple((
        table_alias,
        match_token(AS),
        match_token(LParen),
        select_stmt,
        match_token(RParen),
    ))(i)
    .map(|(i, (alias, _, _, query, _))| {
        (
            i,
            Cte {
                alias,
                query: Box::new(query),
            },
        )
    })
}

fn table_alias(i: Input) -> IResult<TableAlias> {
    alt((
        tuple((
            ident,
            match_token(LParen),
            comma_separated_list1(ident),
            match_token(RParen),
        ))
        .map(|(name, _, columns, _)| TableAlias { name, columns }),
        ident.map(|name| TableAlias {
            name,
            columns: vec![],
        }),
    ))(i)
}

fn order_by_clause(i: Input) -> IResult<Vec<OrderByExpr>> {
    tuple((
        match_token(ORDER),
        match_token(BY),
        comma_separated_list1(order_by_expr),
    ))(i)
    .map(|(i, (_, _, order_by_list))| (i, order_by_list))
}
pub fn order_by_expr(i: Input) -> IResult<OrderByExpr> {
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

        let tokens = tokenize_sql(
            "\
            select a, count(*) \
from (select * from t1) as t2 \
join t3 on t2.a = t3.a \
left join t4 on t3.b = t4.b \
where a > ((1 + 2) * 3) and b < c \
group by a, c \
having count(*) > 5\
order by a, b desc \
limit 1, 2",
        );
        let result = select_stmt(&tokens);
        println!("result: {:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0, vec![]);
        println!("select_stmt: {:#?}", result.1);
    }

    #[test]
    pub fn test_named_windows() {
        use super::select_stmt;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql(
            "\
            select count(*) over w from t window w as (partition by a order by b)",
        );
        let result = select_stmt(&tokens);
        println!("result: {:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(result.0, vec![]);
        println!("select_stmt: {:?}", result.1);
    }
}
