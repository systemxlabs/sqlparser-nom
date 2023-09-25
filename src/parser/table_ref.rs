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
        let Ok((_, op)) = pratt_operator(i) else {
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
    let Some(token) = i.get(0) else {
        return Err("No token found".to_string());
    };
    match token.kind {
        LParen => {
            let (i, right) = pratt_parse(
                i.slice(1..),
                precedence(PrattOp::LParen, AffixKind::Prefix)?,
            )?;

            // next token should be RParen
            let Some(next_token) = i.get(0) else {
                return Err("Expect ')' token".to_string());
            };
            // eat RParen
            let i = i.slice(1..);
            if next_token.kind != RParen {
                return Err("Expect ')' token".to_string());
            }

            // see if there is a alias
            match table_alias(i) {
                Ok((i, alias)) => {
                    let right = match right {
                        TableRef::BaseTable { name, .. } => TableRef::BaseTable {
                            name,
                            alias: Some(alias.clone()),
                        },
                        TableRef::Subquery { subquery, .. } => TableRef::Subquery {
                            subquery,
                            alias: Some(alias.clone()),
                        },
                        TableRef::Join { .. } => {
                            return Err("Joined table should not have an alias".to_string());
                        }
                    };
                    Ok((i, right))
                }
                Err(_) => Ok((i, right)),
            }
        }
        // subquery
        SELECT => {
            let Ok((i, query)) = select_stmt(i) else {
                return Err("can not find prefix subquery".to_string());
            };
            Ok((
                i,
                TableRef::Subquery {
                    subquery: Box::new(query),
                    alias: None,
                },
            ))
        }
        // base table
        Ident => {
            let Ok((i, table_ref)) = base_table(i) else {
                return Err("can not find prefix base table".to_string());
            };
            Ok((i, table_ref))
        }
        _ => Err("First token can't be treated as prefix".to_string()),
    }
}

// find infix table_ref
fn infix(i: Input, left: TableRef) -> Result<(Input, TableRef), String> {
    // find infix operator to get its precedence
    let Ok((i, op)) = pratt_operator(i) else {
        return Err("No infix operator found".to_string());
    };
    match op {
        PrattOp::JoinOp(op) => {
            let (i, right) = pratt_parse(i, precedence(PrattOp::JoinOp(op), AffixKind::Infix)?)?;
            let Ok((i, condition)) = join_condition(i) else {
                return Err("failed to parse join condition".to_string());
            };
            Ok((
                i,
                TableRef::Join {
                    op,
                    condition,
                    left: Box::new(left),
                    right: Box::new(right),
                },
            ))
        }
        _ => {
            return Err("The pratt operator can't be treated as infix".to_string());
        }
    }
}

enum PrattOp {
    JoinOp(JoinOp),
    LParen,
    RParen,
}
fn pratt_operator(i: Input) -> IResult<PrattOp> {
    alt((
        match_token(LParen).map(|_| PrattOp::LParen),
        match_token(RParen).map(|_| PrattOp::RParen),
        join_operator.map(|op| PrattOp::JoinOp(op)),
    ))(i)
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
            PrattOp::JoinOp(_) => Ok(1),
            _ => Err("pratt operator can't be treated as infix".to_string()),
        },
    }
}

fn base_table(i: Input) -> IResult<TableRef> {
    tuple((table_name, opt(table_alias)))(i)
        .map(|(i, (name, alias))| (i, TableRef::BaseTable { name, alias }))
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
        tuple((match_token(CROSS), match_token(JOIN))).map(|(_, _)| JoinOp::CrossJoin),
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
    alt((
        tuple((match_token(AS), ident)).map(|(_, alias)| alias),
        ident,
    ))(i)
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_table_ref() {
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("\
        (select * from t1) as t join t2 on t.a = t2.a left join (t3 right join t4 on t3.c = t4.c) on t2.b = t3.b\
        ");
        let result = super::table_ref(&tokens);
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(
            format!("{}", result.unwrap().1),
            "(((SELECT * FROM t1) AS t INNER JOIN t2 ON (t.a = t2.a)) LEFT OUTER JOIN (t3 RIGHT OUTER JOIN t4 ON (t3.c = t4.c)) ON (t2.b = t3.b))"
        );
    }
}
