use nom::branch::alt;
use nom::combinator::opt;
use nom::sequence::tuple;
use nom::{Parser, Slice};

use super::token::*;
use crate::ast::table_ref::{JoinCondition, JoinOp, TableName, TableRef};
use crate::parser::common::{ident, match_token, AffixKind, MIN_PRECEDENCE};
use crate::parser::error::PError;
use crate::parser::expr::expr;
use crate::parser::statement::select_stmt;
use crate::parser::token::{Token, TokenKind};
use crate::parser::{IResult, Input};

pub fn table_ref(i: Input) -> IResult<TableRef> {
    match pratt_parse(i, MIN_PRECEDENCE) {
        Ok(r) => Ok(r),
        Err(e) => Err(nom::Err::Error(PError(e))),
    }
}

fn pratt_parse(i: Input, lbp: u32) -> Result<(Input, TableRef), String> {
    // find a prefix table_ref
    let (mut i, mut left) = prefix(i)?;
    loop {
        let Some(op) = peek_operator(i) else {
            break;
        };
        let Ok(bp) = precedence(op, AffixKind::Infix) else {
            // end of table_ref
            break;
        };
        // compare to infix precedence
        if lbp >= bp {
            // if prefix precedence is greater than infix, then break
            break;
        }
        // find infix table_ref with prefix table_ref
        match infix(i, left.clone()) {
            Ok(r) => {
                i = r.0;
                left = r.1
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok((i, left))
}

// find prefix table_ref
fn prefix(i: Input) -> Result<(Input, TableRef), String> {
    // TODO lparen
    alt((subquery, base_table))(i).or_else(|e| Err("Can't find prefix expr".to_string()))
}

// find infix table_ref
fn infix(i: Input, left: TableRef) -> Result<(Input, TableRef), String> {
    // find infix operator to get its precedence
    let Some(op) = peek_operator(i) else {
        return Err("No infix operator found".to_string());
    };
    let i = i.slice(1..);
    todo!()
}

enum PrattOp {
    JoinOp,
    LParen,
    RParen,
}

fn precedence(op: PrattOp, affix: AffixKind) -> Result<u32, String> {
    match affix {
        // prefix precedence should be grater than infix
        AffixKind::Prefix => match op {
            PrattOp::LParen => Ok(0),
            _ => Err("pratt operator can't be treated as prefix".to_string()),
        },
        AffixKind::Infix => match op {
            PrattOp::RParen => Ok(0),
            PrattOp::JoinOp => Ok(1),
            _ => Err("pratt operator can't be treated as infix".to_string()),
        },
    }
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

fn peek_operator(i: Input) -> Option<PrattOp> {
    if let Some(token) = i.get(0) {
        if matches!(token.kind, LParen) {
            return Some(PrattOp::LParen);
        }
        if matches!(token.kind, RParen) {
            return Some(PrattOp::RParen);
        }
    }
    let res = join_operator(i);
    return match res {
        Ok((_, _)) => Some(PrattOp::JoinOp),
        Err(_) => None,
    };
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
