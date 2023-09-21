use super::Ident;

#[derive(Debug, Clone)]
pub enum Expr {
    ColumnRef {
        database: Option<Ident>,
        table: Option<Ident>,
        column: Ident,
    },
    Literal(Literal),
    Alias {
        expr: Box<Expr>,
        alias: Ident,
    },
    UnaryOp {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    BinaryOp {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
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
            Self::Alias { expr, alias } => write!(f, "{} AS {}", expr, alias),
            Self::UnaryOp { op, expr } => write!(f, "{}{}", op, expr),
            Self::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op, right),
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
