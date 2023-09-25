use nom::combinator::opt;
use nom::Parser;
use nom::{branch::alt, sequence::tuple};

use crate::ast::expr::Expr;
use crate::ast::set_expr::{NamedWindowDef, SelectItem, SetExpr, WildcardOptions};
use crate::parser::expr::window_spec;
use crate::parser::table_ref::table_ref;
use crate::parser::token::*;

use super::common::{comma_separated_list1, ident, match_text};
use super::expr::expr;
use super::{common::match_token, IResult, Input};

pub fn select_set_expr(i: Input) -> IResult<SetExpr> {
    tuple((
        match_token(SELECT),
        opt(match_token(DISTINCT)),
        comma_separated_list1(select_item),
        opt(tuple((match_token(FROM), table_ref))),
        opt(where_clause),
        opt(group_by_clause),
        opt(having_clause),
        opt(window_clause),
    ))(i)
    .map(
        |(i, (_, distinct, projection, from, selection, group_by, having, named_windows))| {
            (
                i,
                SetExpr::Select {
                    distinct: distinct.is_some(),
                    projection,
                    from: from.map(|(_, from)| from),
                    selection,
                    group_by: group_by.unwrap_or(vec![]),
                    having,
                    named_windows: named_windows.map_or(vec![], |v| v),
                },
            )
        },
    )
}

fn select_item(i: Input) -> IResult<SelectItem> {
    alt((
        tuple((match_text("*"), wildcard_options))
            .map(|(_, options)| SelectItem::Wildcard(options)),
        tuple((expr, match_token(AS), ident))
            .map(|(expr, _, alias)| SelectItem::ExprWithAlias { expr, alias }),
        expr.map(|expr| SelectItem::UnnamedExpr(expr)),
    ))(i)
}

fn wildcard_options(i: Input) -> IResult<WildcardOptions> {
    tuple((
        opt(tuple((
            match_token(EXCLUDE),
            match_token(LParen),
            comma_separated_list1(ident),
            match_token(RParen),
        ))),
        opt(tuple((
            match_token(EXCEPT),
            match_token(LParen),
            comma_separated_list1(ident),
            match_token(RParen),
        ))),
    ))(i)
    .map(|(i, (exclude, except))| {
        let exclude: Vec<crate::ast::Ident> = exclude.map_or(vec![], |(_, _, cols, _)| cols);
        let except: Vec<crate::ast::Ident> = except.map_or(vec![], |(_, _, cols, _)| cols);
        (i, WildcardOptions { exclude, except })
    })
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

fn window_clause(i: Input) -> IResult<Vec<NamedWindowDef>> {
    tuple((match_token(WINDOW), comma_separated_list1(named_window_def)))(i)
        .map(|(i, (_, defs))| (i, defs))
}

fn named_window_def(i: Input) -> IResult<NamedWindowDef> {
    tuple((
        ident,
        match_token(AS),
        match_token(LParen),
        window_spec,
        match_token(RParen),
    ))(i)
    .map(|(i, (name, _, _, spec, _))| (i, NamedWindowDef { name, spec }))
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
