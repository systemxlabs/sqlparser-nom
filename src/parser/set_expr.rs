use nom::Parser;
use nom::{branch::alt, sequence::tuple};

use crate::ast::set_expr::{SelectItem, SetExpr};
use crate::parser::token::*;

use super::common::{comma_separated_list1, ident, match_text};
use super::expr::column_ref;
use super::{common::match_token, IResult, Input};

pub fn select_set_expr(i: Input) -> IResult<SetExpr> {
    tuple((
        match_token(SELECT),
        comma_separated_list1(select_item),
        match_token(FROM),
        ident,
    ))(i)
    .map(|(i, (_, projection, _, from))| {
        Ok((
            i,
            SetExpr::Select {
                projection,
                from,
                where_clause: None,
                group_by: vec![],
            },
        ))
    })?
}

pub fn select_item(i: Input) -> IResult<SelectItem> {
    alt((
        match_text("*").map(|_| SelectItem::Wildcard),
        tuple((column_ref, match_token(AS), ident))
            .map(|(expr, _, alias)| SelectItem::ExprWithAlias { expr, alias }),
        column_ref.map(|expr| SelectItem::UnnamedExpr(expr)),
    ))(i)
}

#[cfg(test)]
mod tests {

    #[test]
    pub fn test_select_item_list() {
        use crate::parser::common::comma_separated_list1;
        use crate::parser::{set_expr::select_item, tokenize_sql};

        let tokens = tokenize_sql("*, t1.a, c as d");
        let items = comma_separated_list1(select_item)(&tokens);
        assert!(items.is_ok());
        assert_eq!(
            format!(
                "{}",
                items
                    .unwrap()
                    .1
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            "*, t1.a, c AS d"
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
