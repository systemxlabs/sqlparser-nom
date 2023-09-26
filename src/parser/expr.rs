use nom::combinator::opt;
use nom::{branch::alt, sequence::tuple};
use nom::{Parser, Slice};

use crate::ast::expr::{BinaryOp, Expr, FunctionArg, Literal, UnaryOp, Window, WindowSpec};
use crate::ast::statement::SelectStatement;
use crate::parser::common::{comma_separated_list0, AffixKind, MIN_PRECEDENCE};
use crate::parser::error::PError;
use crate::parser::statement::{order_by_expr, select_stmt};
use crate::parser::token::{
    LParen, RParen, Token, TokenKind, BY, EXISTS, IN, NOT, ORDER, OVER, PARTITION,
};

use super::common::comma_separated_list1;
use super::{
    common::{ident, match_token},
    IResult, Input,
};

pub fn expr(i: Input) -> IResult<Expr> {
    match pratt_parse(i, MIN_PRECEDENCE) {
        Ok((i, pratt_expr)) => Ok((i, pratt_expr.into_expr())),
        Err(e) => Err(nom::Err::Error(PError(e))),
    }
}

/// pratt parsing algorithm
fn pratt_parse(i: Input, lbp: u32) -> Result<(Input, PrattExpr), String> {
    // find a prefix expr
    let (mut i, mut pratt_left) = prefix(i)?;
    loop {
        let Some(token) = i.get(0) else {
            break;
        };
        let Ok(bp) = precedence(token, AffixKind::Infix) else {
            // end of expr
            break;
        };
        // compare to infix precedence
        if lbp >= bp {
            // if prefix precedence is greater than infix, then break
            break;
        }
        // find infix expr with prefix expr
        match infix(i, pratt_left.clone()) {
            Ok(r) => {
                i = r.0;
                pratt_left = r.1
            }
            Err(e) => {
                return Err(e);
            }
        }
    }
    Ok((i, pratt_left))
}

// find prefix expr
fn prefix(i: Input) -> Result<(Input, PrattExpr), String> {
    let Some(token) = i.get(0) else {
        return Err("No token found".to_string());
    };
    match token.kind {
        TokenKind::LParen => {
            let (i, right) = pratt_parse(i.slice(1..), precedence(token, AffixKind::Prefix)?)?;
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
            i.slice(1..),
            PrattExpr::Expr(Expr::Literal(Literal::UnsignedInteger(
                token.text().parse::<usize>().unwrap(),
            ))),
        )),
        TokenKind::Ident => {
            let Ok((i, expr)) = alt((function_expr, column_ref_expr))(i) else {
                return Err("can not find prefix expr".to_string());
            };
            Ok((i, PrattExpr::Expr(expr)))
        }
        TokenKind::NOT => {
            let Ok((i, expr)) = exists_expr(i) else {
                return Err("can not find prefix expr".to_string());
            };
            Ok((i, PrattExpr::Expr(expr)))
        }
        TokenKind::EXISTS => {
            let Ok((i, expr)) = exists_expr(i) else {
                return Err("can not find prefix expr".to_string());
            };
            Ok((i, PrattExpr::Expr(expr)))
        }
        TokenKind::Plus => {
            let (i, pratt_expr) = pratt_parse(i.slice(1..), precedence(token, AffixKind::Prefix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::UnaryOp {
                    op: UnaryOp::Plus,
                    expr: Box::new(pratt_expr.into_expr()),
                }),
            ))
        }
        TokenKind::Minus => {
            let (i, pratt_expr) = pratt_parse(i.slice(1..), precedence(token, AffixKind::Prefix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::UnaryOp {
                    op: UnaryOp::Minus,
                    expr: Box::new(pratt_expr.into_expr()),
                }),
            ))
        }
        _ => Err("First token can't be treated as prefix".to_string()),
    }
}

fn infix(i: Input, pratt_left: PrattExpr) -> Result<(Input, PrattExpr), String> {
    // find infix operator to get its precedence
    let Some(token) = i.get(0) else {
        return Err("No token found".to_string());
    };
    let i = i.slice(1..);
    match token.kind {
        TokenKind::Plus => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Add,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::Minus => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Sub,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::Multiply => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Mul,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::Divide => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Div,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::Gt => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Gt,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::Lt => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Lt,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::GtEq => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::GtEq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::LtEq => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::LtEq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::Eq => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Eq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::NotEq => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::NotEq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::AND => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::And,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        TokenKind::OR => {
            let (i, pratt_right) = pratt_parse(i, precedence(token, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Or,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        _ => {
            return Err("The token can't be treated as infix".to_string());
        }
    }
}

#[derive(Debug, Clone)]
enum PrattExpr {
    Expr(Expr),
    Subquery(SelectStatement),
}
impl PrattExpr {
    pub fn into_expr(self) -> Expr {
        match self {
            PrattExpr::Expr(expr) => expr,
            _ => panic!("PrattExpr is not expr"),
        }
    }
    pub fn into_subquery(self) -> SelectStatement {
        match self {
            PrattExpr::Subquery(stmt) => stmt,
            _ => panic!("PrattExpr is not subquery"),
        }
    }
}

fn precedence(token: &Token, affix: AffixKind) -> Result<u32, String> {
    match affix {
        // prefix precedence should be grater than infix
        AffixKind::Prefix => match token.kind {
            TokenKind::LParen => Ok(0),
            TokenKind::Plus | TokenKind::Minus => Ok(300),
            _ => Err("token can't be treated as prefix".to_string()),
        },
        AffixKind::Infix => match token.kind {
            TokenKind::RParen => Ok(0),
            TokenKind::OR => Ok(8),
            TokenKind::AND => Ok(9),
            TokenKind::Gt
            | TokenKind::Lt
            | TokenKind::GtEq
            | TokenKind::LtEq
            | TokenKind::Eq
            | TokenKind::NotEq => Ok(10),
            TokenKind::Plus | TokenKind::Minus => Ok(11),
            TokenKind::Multiply | TokenKind::Divide => Ok(12),
            _ => Err("token can't be treated as infix".to_string()),
        },
    }
}

fn column_ref_expr(i: Input) -> IResult<Expr> {
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

fn function_expr(i: Input) -> IResult<Expr> {
    tuple((
        ident,
        match_token(TokenKind::LParen),
        opt(match_token(TokenKind::DISTINCT)),
        comma_separated_list0(function_arg),
        match_token(TokenKind::RParen),
        opt(window),
    ))(i)
    .map(|(i, (name, _, distinct, args, _, over))| {
        (
            i,
            Expr::Function {
                name,
                distinct: distinct.is_some(),
                args,
                over,
            },
        )
    })
}

fn function_arg(i: Input) -> IResult<FunctionArg> {
    alt((
        match_token(TokenKind::Multiply).map(|_| FunctionArg::Wildcard),
        expr.map(|expr| FunctionArg::Expr(expr)),
    ))(i)
}

fn exists_expr(i: Input) -> IResult<Expr> {
    tuple((
        opt(match_token(NOT)),
        match_token(EXISTS),
        match_token(LParen),
        select_stmt,
        match_token(RParen),
    ))(i)
    .map(|(i, (not, _, _, subquery, _))| {
        (
            i,
            Expr::Exists {
                not: not.is_some(),
                subquery: Box::new(subquery),
            },
        )
    })
}

fn window(i: Input) -> IResult<Window> {
    alt((
        tuple((match_token(OVER), ident)).map(|(_, window_ref)| Window::WindowRef(window_ref)),
        tuple((
            match_token(OVER),
            match_token(LParen),
            window_spec,
            match_token(RParen),
        ))
        .map(|(_, _, spec, _)| Window::WindowSpec(spec)),
    ))(i)
}

pub fn window_spec(i: Input) -> IResult<WindowSpec> {
    tuple((
        opt(tuple((
            match_token(PARTITION),
            match_token(BY),
            comma_separated_list1(expr),
        ))),
        opt(tuple((
            match_token(ORDER),
            match_token(BY),
            comma_separated_list1(order_by_expr),
        ))),
    ))(i)
    .map(|(i, (partition_by, order_by))| {
        (
            i,
            WindowSpec {
                partition_by: partition_by.map_or(vec![], |p| p.2),
                order_by: order_by.map_or(vec![], |o| o.2),
            },
        )
    })
}

mod tests {

    #[test]
    pub fn test_expr() {
        use super::*;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("1*(2-3)+4/2 + t1.a");
        let result = expr(&tokens).unwrap();
        println!("expr: {}", result.1);

        let tokens = tokenize_sql("t1.a != 1 or t1.b > 2 and c = 3");
        let result = expr(&tokens).unwrap();
        println!("expr: {}", result.1);
    }

    #[test]
    pub fn test_column_ref() {
        use super::*;
        use crate::ast::Ident;
        use crate::parser::expr::column_ref_expr;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("t1.b");
        let column_ref = column_ref_expr(&tokens);
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

    #[test]
    pub fn test_function() {
        use super::*;
        use crate::parser::expr::function_expr;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("count(distinct a) over (partition by a order by b)");
        let result = function_expr(&tokens);
        println!("{:?}", result);
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.0.is_empty());
        assert!(matches!(result.1, Expr::Function { .. }));
        println!("{}", result.1);
    }
}
