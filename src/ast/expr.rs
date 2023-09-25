use super::Ident;
use crate::ast::statement::OrderByExpr;

#[derive(Debug, Clone)]
pub enum Expr {
    ColumnRef {
        database: Option<Ident>,
        table: Option<Ident>,
        column: Ident,
    },
    Literal(Literal),
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    Function {
        name: Ident,
        distinct: bool,
        args: Vec<FunctionArg>,
        over: Option<Window>,
    },
}
impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ColumnRef {
                database,
                table,
                column,
            } => {
                if let Some(database) = database {
                    write!(f, "{}.", database)?;
                }
                if let Some(table) = table {
                    write!(f, "{}.", table)?;
                }
                write!(f, "{}", column)
            }
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::UnaryOp { op, expr } => write!(f, "{}{}", op, expr),
            Self::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op, right),
            Self::Function {
                name,
                distinct,
                args,
                over,
            } => {
                write!(
                    f,
                    "{}({}{})",
                    name,
                    if *distinct { "DISTINCT " } else { "" },
                    args.iter()
                        .map(|a| a.to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                )?;
                if let Some(window) = over {
                    write!(f, " OVER {window}")?;
                }
                Ok(())
            }
        }
    }
}

/// Binary operators
#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
    // + - * / %
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    // > < >= <= = !=
    Gt,
    Lt,
    GtEq,
    LtEq,
    Eq,
    NotEq,

    // and or
    And,
    Or,
}
impl std::fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Add => write!(f, "+"),
            Self::Sub => write!(f, "-"),
            Self::Mul => write!(f, "*"),
            Self::Div => write!(f, "/"),
            Self::Mod => write!(f, "%"),
            Self::Gt => write!(f, ">"),
            Self::Lt => write!(f, "<"),
            Self::GtEq => write!(f, ">="),
            Self::LtEq => write!(f, "<="),
            Self::Eq => write!(f, "="),
            Self::NotEq => write!(f, "!="),
            Self::And => write!(f, "AND"),
            Self::Or => write!(f, "OR"),
        }
    }
}

/// Literal values
#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    UnsignedInteger(usize),
    UnsignedFloat(f64),
}
impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "'{}'", s),
            Self::UnsignedInteger(i) => write!(f, "{}", i),
            Self::UnsignedFloat(fl) => write!(f, "{}", fl),
        }
    }
}

/// Unary operators
#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Plus,
    Minus,
}

impl std::fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum FunctionArg {
    Wildcard,
    Expr(Expr),
}
impl std::fmt::Display for FunctionArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Wildcard => write!(f, "*"),
            Self::Expr(expr) => write!(f, "{}", expr),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Window {
    WindowRef(Ident),
    WindowSpec(WindowSpec),
}
impl std::fmt::Display for Window {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WindowRef(name) => write!(f, "{}", name),
            Self::WindowSpec(spec) => write!(f, "({})", spec),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WindowSpec {
    pub partition_by: Vec<Expr>,
    pub order_by: Vec<OrderByExpr>,
}

impl std::fmt::Display for WindowSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut delim = "";
        if !self.partition_by.is_empty() {
            delim = " ";
            write!(
                f,
                "PARTITION BY {}",
                self.partition_by
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        if !self.order_by.is_empty() {
            f.write_str(delim)?;
            // delim = " ";
            write!(
                f,
                "ORDER BY {}",
                self.order_by
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
