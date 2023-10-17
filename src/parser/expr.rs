use nom::combinator::opt;
use nom::{branch::alt, sequence::tuple};
use nom::{Parser, Slice};

use crate::ast::expr::{BinaryOp, Expr, FunctionArg, Literal, UnaryOp, Window, WindowSpec};
use crate::parser::common::{comma_separated_list0, AffixKind, MIN_PRECEDENCE};
use crate::parser::error::PError;
use crate::parser::statement::{order_by_expr, select_stmt};
use crate::parser::token::*;

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
        let Ok((_, op)) = pratt_operator(i) else {
            break;
        };
        let Ok(bp) = precedence(op, AffixKind::Infix) else {
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
        LParen => {
            let (i, right) = pratt_parse(i.slice(1..), MIN_PRECEDENCE)?;
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
        LiteralInteger => Ok((
            i.slice(1..),
            PrattExpr::Expr(Expr::Literal(Literal::UnsignedInteger(
                token.text().parse::<usize>().unwrap(),
            ))),
        )),
        Ident => {
            let Ok((i, expr)) = alt((function_expr, column_ref_expr))(i) else {
                return Err("can not find prefix expr".to_string());
            };
            Ok((i, PrattExpr::Expr(expr)))
        }
        NOT => {
            let Ok((i, expr)) = exists_expr(i) else {
                return Err("can not find prefix expr".to_string());
            };
            Ok((i, PrattExpr::Expr(expr)))
        }
        EXISTS => {
            let Ok((i, expr)) = exists_expr(i) else {
                return Err("can not find prefix expr".to_string());
            };
            Ok((i, PrattExpr::Expr(expr)))
        }
        Plus => {
            let (i, pratt_expr) =
                pratt_parse(i.slice(1..), precedence(PrattOp::Plus, AffixKind::Prefix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::UnaryOp {
                    op: UnaryOp::Plus,
                    expr: Box::new(pratt_expr.into_expr()),
                }),
            ))
        }
        Minus => {
            let (i, pratt_expr) =
                pratt_parse(i.slice(1..), precedence(PrattOp::Minus, AffixKind::Prefix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::UnaryOp {
                    op: UnaryOp::Minus,
                    expr: Box::new(pratt_expr.into_expr()),
                }),
            ))
        }
        SELECT => {
            let Ok((i, stmt)) = select_stmt(i) else {
                return Err("can not parse select statement".to_string());
            };
            Ok((i, PrattExpr::Expr(Expr::Subquery(Box::new(stmt)))))
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
        Plus => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::Plus, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Add,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        Minus => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::Minus, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Sub,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        Multiply => {
            let (i, pratt_right) =
                pratt_parse(i, precedence(PrattOp::Multiply, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Mul,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        Divide => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::Divide, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Div,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        Gt => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::Gt, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Gt,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        Lt => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::Lt, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Lt,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        GtEq => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::GtEq, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::GtEq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        LtEq => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::LtEq, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::LtEq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        Eq => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::Eq, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Eq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        NotEq => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::NotEq, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::NotEq,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        AND => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::And, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::And,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        OR => {
            let (i, pratt_right) = pratt_parse(i, precedence(PrattOp::Or, AffixKind::Infix)?)?;
            Ok((
                i,
                PrattExpr::Expr(Expr::BinaryOp {
                    left: Box::new(pratt_left.into_expr()),
                    op: BinaryOp::Or,
                    right: Box::new(pratt_right.into_expr()),
                }),
            ))
        }
        // postfix
        IN => {
            if let Ok((i, subquery)) =
                tuple((match_token(LParen), select_stmt, match_token(RParen)))(i)
                    .map(|(i, (_, query, _))| (i, query))
            {
                Ok((
                    i,
                    PrattExpr::Expr(Expr::InSubquery {
                        not: false,
                        expr: Box::new(pratt_left.into_expr()),
                        subquery: Box::new(subquery),
                    }),
                ))
            } else if let Ok((i, exprs)) = tuple((
                match_token(LParen),
                comma_separated_list0(expr),
                match_token(RParen),
            ))(i)
            .map(|(i, (_, exprs, _))| (i, exprs))
            {
                Ok((
                    i,
                    PrattExpr::Expr(Expr::InList {
                        not: false,
                        expr: Box::new(pratt_left.into_expr()),
                        list: exprs,
                    }),
                ))
            } else {
                return Err(format!("Failed to parse InSubquery or InList"));
            }
        }
        NOT => {
            let Some(token) = i.get(0) else {
                return Err("No token found".to_string());
            };
            match token.kind {
                IN => {
                    let i = i.slice(1..);
                    if let Ok((i, subquery)) =
                        tuple((match_token(LParen), select_stmt, match_token(RParen)))(i)
                            .map(|(i, (_, query, _))| (i, query))
                    {
                        Ok((
                            i,
                            PrattExpr::Expr(Expr::InSubquery {
                                not: true,
                                expr: Box::new(pratt_left.into_expr()),
                                subquery: Box::new(subquery),
                            }),
                        ))
                    } else if let Ok((i, exprs)) = tuple((
                        match_token(LParen),
                        comma_separated_list0(expr),
                        match_token(RParen),
                    ))(i)
                    .map(|(i, (_, exprs, _))| (i, exprs))
                    {
                        Ok((
                            i,
                            PrattExpr::Expr(Expr::InList {
                                not: true,
                                expr: Box::new(pratt_left.into_expr()),
                                list: exprs,
                            }),
                        ))
                    } else {
                        return Err(format!("Failed to parse InSubquery or InList"));
                    }
                }
                _ => {
                    return Err(format!("Not support pratt operator: not {}", token.kind));
                }
            }
        }
        _ => {
            return Err("The token can't be treated as infix".to_string());
        }
    }
}

/// This is a general representation of pratt expression.
#[derive(Debug, Clone)]
enum PrattExpr {
    Expr(Expr),
}
impl PrattExpr {
    pub fn into_expr(self) -> Expr {
        match self {
            PrattExpr::Expr(expr) => expr,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum PrattOp {
    // + -
    Plus,
    Minus,
    // * /
    Multiply,
    Divide,
    // > < >= <= = != <>
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,
    // and or
    And,
    Or,
    // (not) in
    In,
}

fn pratt_operator(i: Input) -> IResult<PrattOp> {
    alt((
        match_token(Plus).map(|_| PrattOp::Plus),
        match_token(Minus).map(|_| PrattOp::Minus),
        match_token(Multiply).map(|_| PrattOp::Multiply),
        match_token(Gt).map(|_| PrattOp::Gt),
        match_token(Lt).map(|_| PrattOp::Lt),
        match_token(GtEq).map(|_| PrattOp::GtEq),
        match_token(LtEq).map(|_| PrattOp::LtEq),
        match_token(Eq).map(|_| PrattOp::Eq),
        match_token(NotEq).map(|_| PrattOp::NotEq),
        match_token(AND).map(|_| PrattOp::And),
        match_token(OR).map(|_| PrattOp::Or),
        tuple((opt(match_token(NOT)), match_token(IN))).map(|_| PrattOp::In),
    ))(i)
}

fn precedence(op: PrattOp, affix: AffixKind) -> Result<u32, String> {
    match affix {
        // prefix precedence should be grater than infix
        AffixKind::Prefix => match op {
            PrattOp::Plus | PrattOp::Minus => Ok(300),
            _ => Err(format!("pratt op {:?} can't be treated as prefix", op)),
        },
        AffixKind::Infix => match op {
            PrattOp::In => Ok(7),
            PrattOp::Or => Ok(8),
            PrattOp::And => Ok(9),
            PrattOp::Gt
            | PrattOp::Lt
            | PrattOp::GtEq
            | PrattOp::LtEq
            | PrattOp::Eq
            | PrattOp::NotEq => Ok(10),
            PrattOp::Plus | PrattOp::Minus => Ok(11),
            PrattOp::Multiply | PrattOp::Divide => Ok(12),
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

        let tokens = tokenize_sql("t1.a in (select 1)");
        let result = expr(&tokens).unwrap();
        println!("expr: {}", result.1);

        let tokens = tokenize_sql("t1.a in (1, 2, 3)");
        let result = expr(&tokens).unwrap();
        println!("expr: {}", result.1);
    }

    #[test]
    pub fn test_in_subquery() {
        use super::*;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("t1.a in (select 1)");
        let result = expr(&tokens).unwrap();
        assert_eq!(result.0.len(), 0);
        assert_eq!(format!("{}", result.1), "t1.a IN (SELECT 1)");
    }

    #[test]
    pub fn test_in_list() {
        use super::*;
        use crate::parser::tokenize_sql;

        let tokens = tokenize_sql("t1.a in (1, 2, 3)");
        let result = expr(&tokens).unwrap();
        assert_eq!(result.0.len(), 0);
        assert_eq!(format!("{}", result.1), "t1.a IN (1, 2, 3)");
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
