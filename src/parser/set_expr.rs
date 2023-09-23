use nom::combinator::opt;
use nom::Parser;
use nom::{branch::alt, sequence::tuple};

use crate::ast::expr::Expr;
use crate::ast::set_expr::{SelectItem, SetExpr, TableName, TableRef};
use crate::parser::statement::select_stmt;
use crate::parser::token::*;

use super::common::{comma_separated_list1, ident, match_text};
use super::expr::expr;
use super::{common::match_token, IResult, Input};

pub fn select_set_expr(i: Input) -> IResult<SetExpr> {
    tuple((
        match_token(SELECT),
        comma_separated_list1(select_item),
        match_token(FROM),
        opt(table_ref),
        opt(where_clause),
        opt(group_by_clause),
        opt(having_clause),
    ))(i)
    .map(
        |(i, (_, projection, _, from, selection, group_by, having))| {
            (
                i,
                SetExpr::Select {
                    projection,
                    from,
                    selection,
                    group_by: group_by.unwrap_or(vec![]),
                    having,
                },
            )
        },
    )
}

fn select_item(i: Input) -> IResult<SelectItem> {
    alt((
        match_text("*").map(|_| SelectItem::Wildcard),
        tuple((expr, match_token(AS), ident))
            .map(|(expr, _, alias)| SelectItem::ExprWithAlias { expr, alias }),
        expr.map(|expr| SelectItem::UnnamedExpr(expr)),
    ))(i)
}

fn where_clause(i: Input) -> IResult<Expr> {
    tuple((match_token(WHERE), expr))(i).map(|(i, (_, expr))| (i, expr))
}

fn group_by_clause(i: Input) -> IResult<Vec<Expr>> {
    tuple((
        match_token(GROUP),
        match_token(BY),
        comma_separated_list1(expr),
    ))(i)
    .map(|(i, (_, _, group_by_list))| (i, group_by_list))
}

fn having_clause(i: Input) -> IResult<Expr> {
    tuple((match_token(HAVING), expr))(i).map(|(i, (_, having))| (i, having))
}

fn table_ref(i: Input) -> IResult<TableRef> {
    alt((
        subquery,
        tuple((table_name, opt(table_alias)))
            .map(|(name, alias)| TableRef::BaseTable { name, alias }),
    ))(i)
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

    #[test]
    pub fn test_select_item_list() {
        use crate::parser::common::comma_separated_list1;
        use crate::parser::{set_expr::select_item, tokenize_sql};

        let tokens = tokenize_sql("*, t1.a, c as d, count(e)");
        let result = comma_separated_list1(select_item)(&tokens);
        println!("result: {:?}", result);
        assert!(result.is_ok());
        assert_eq!(
            format!(
                "{}",
                result
                    .unwrap()
                    .1
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            "*, t1.a, c AS d, count(e)"
        );
    }

    #[test]
    pub fn test_select_set_expr() {
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("select *, t1.a, c as d from t1");
        let items = super::select_set_expr(&tokens);
        assert!(items.is_ok());
        assert_eq!(
            format!("{}", items.unwrap().1),
            "SELECT *, t1.a, c AS d FROM t1"
        );
    }
}
