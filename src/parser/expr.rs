use nom::{branch::alt, sequence::tuple};
use nom::{Parser, Slice};

use crate::ast::expr::{BinaryOp, Expr, Literal, UnaryOp};
use crate::parser::error::PError;
use crate::parser::token::{Token, TokenKind};

use super::{
    common::{ident, match_token},
    IResult, Input,
};

const MIN_PRECEDENCE: u32 = 0;

pub fn expr(i: Input) -> IResult<Expr> {
    match pratt_parse(i, MIN_PRECEDENCE) {
        Ok(r) => Ok(r),
        Err(e) => Err(nom::Err::Error(PError(e))),
    }
}

/// pratt parsing algorithm
fn pratt_parse(i: Input, lbp: u32) -> Result<(Input, Expr), String> {
    // find a prefix expr
    let (mut i, mut left) = prefix(i)?;
    loop {
        let Some(token) = i.get(0) else {
            break;
        };
        // compare to infix precedence
        let bp = precedence(token, AffixKind::Infix);
        if lbp >= bp {
            // if prefix precedence is greater than infix, then break
            break;
        }
        // find infix expr with prefix expr
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

// find prefix expr
fn prefix(i: Input) -> Result<(Input, Expr), String> {
    let Some(token) = i.get(0) else {
        return Err("No token found".to_string());
    };
    let i = i.slice(1..);

    match token.kind {
        TokenKind::LParen => {
            let (i, right) = pratt_parse(i, precedence(token, AffixKind::Prefix))?;
            // next token should be RParen
            let Some(next_token) = i.get(0) else {
                return Err("Expect ')' token".to_string());
            };
            // eat RParen
            let i = i.slice(1..);
            match next_token.kind {
                TokenKind::RParen => Ok((i, right)),
                _ => Err("Expect ')' token".to_string()),
            }
        }
        TokenKind::LiteralInteger => Ok((
            i,
            Expr::Literal(Literal::UnsignedInteger(
                token.text().parse::<usize>().unwrap(),
            )),
        )),
        TokenKind::Plus => {
            let (i, expr) = pratt_parse(i, precedence(token, AffixKind::Prefix))?;
            Ok((
                i,
                Expr::UnaryOp {
                    op: UnaryOp::Plus,
                    expr: Box::new(expr),
                },
            ))
        }
        TokenKind::Minus => {
            let (i, expr) = pratt_parse(i, precedence(token, AffixKind::Prefix))?;
            Ok((
                i,
                Expr::UnaryOp {
                    op: UnaryOp::Minus,
                    expr: Box::new(expr),
                },
            ))
        }
        _ => Err("First token can't be treated as prefix".to_string()),
    }
}

fn infix(i: Input, left: Expr) -> Result<(Input, Expr), String> {
    // find infix operator to get its precedence
    let Some(token) = i.get(0) else {
        return Err("No token found".to_string());
    };
    let i = i.slice(1..);
    match token.kind {
        TokenKind::Plus => {
            let (i, right) = pratt_parse(i, precedence(token, AffixKind::Infix))?;
            Ok((
                i,
                Expr::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Add,
                    right: Box::new(right),
                },
            ))
        }
        TokenKind::Minus => {
            let (i, right) = pratt_parse(i, precedence(token, AffixKind::Infix))?;
            Ok((
                i,
                Expr::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Sub,
                    right: Box::new(right),
                },
            ))
        }
        TokenKind::Multiply => {
            let (i, right) = pratt_parse(i, precedence(token, AffixKind::Infix))?;
            Ok((
                i,
                Expr::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Mul,
                    right: Box::new(right),
                },
            ))
        }
        TokenKind::Divide => {
            let (i, right) = pratt_parse(i, precedence(token, AffixKind::Infix))?;
            Ok((
                i,
                Expr::BinaryOp {
                    left: Box::new(left),
                    op: BinaryOp::Div,
                    right: Box::new(right),
                },
            ))
        }
        _ => {
            return Err("The token can't be treated as infix".to_string());
        }
    }
}

enum AffixKind {
    Prefix,
    Infix,
}

fn precedence(token: &Token, affix: AffixKind) -> u32 {
    match affix {
        // prefix precedence should be grater than infix
        AffixKind::Prefix => match token.kind {
            TokenKind::LParen => 0,
            TokenKind::Plus | TokenKind::Minus => 300,
            TokenKind::LiteralInteger => 200,
            _ => unreachable!(),
        },
        AffixKind::Infix => match token.kind {
            TokenKind::RParen => 0,
            TokenKind::Plus | TokenKind::Minus => 10,
            TokenKind::Multiply | TokenKind::Divide => 20,
            _ => unreachable!(),
        },
    }
}

pub fn column_ref(i: Input) -> IResult<Expr> {
    alt((
        tuple((
            ident,
            match_token(TokenKind::Dot),
            ident,
            match_token(TokenKind::Dot),
            ident,
        ))
        .map(|(database, _, table, _, column)| Expr::ColumnRef {
            database: Some(database),
            table: Some(table),
            column,
        }),
        tuple((ident, match_token(TokenKind::Dot), ident)).map(|(table, _, column)| {
            Expr::ColumnRef {
                database: None,
                table: Some(table),
                column,
            }
        }),
        ident.map(|column| Expr::ColumnRef {
            database: None,
            table: None,
            column,
        }),
    ))(i)
}

mod tests {

    #[test]
    pub fn test_expr() {
        use super::*;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("1*(2-3)+4/2");
        let result = expr(&tokens).unwrap();
        println!("expr: {}", result.1);
    }

    #[test]
    pub fn test_column_ref() {
        use super::*;
        use crate::ast::Ident;
        use crate::parser::expr::column_ref;
        use crate::parser::tokenize_sql;

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
}
