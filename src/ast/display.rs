use super::{
    BinaryOp, Expr, Ident, Literal, OrderByExpr, SelectItem, SelectStatement, SetExpr, UnaryOp,
};
use std::fmt::Display;

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.value)
    }
}

impl Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(s) => write!(f, "'{}'", s),
            Self::UnsignedInteger(i) => write!(f, "{}", i),
            Self::UnsignedFloat(fl) => write!(f, "{}", fl),
        }
    }
}

impl Display for BinaryOp {
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

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "{}", ident),
            Self::CompoundIdentifier(idents) => write!(
                f,
                "{}",
                idents
                    .iter()
                    .map(|p| p.to_string())
                    .collect::<Vec<_>>()
                    .join("."),
            ),
            Self::Literal(literal) => write!(f, "{}", literal),
            Self::Alias { expr, alias } => write!(f, "{} AS {}", expr, alias),
            Self::UnaryOp { op, expr } => write!(f, "{}{}", op, expr),
            Self::BinaryOp { left, op, right } => write!(f, "({} {} {})", left, op, right),
        }
    }
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Plus => write!(f, "+"),
            Self::Minus => write!(f, "-"),
        }
    }
}

impl Display for SelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnnamedExpr(expr) => write!(f, "{}", expr),
            Self::ExprWithAlias { expr, alias } => write!(f, "{} AS {}", expr, alias),
            Self::Wildcard => write!(f, "*"),
        }
    }
}

impl Display for SetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Select {
                projection,
                from,
                where_clause,
                group_by,
            } => {
                write!(
                    f,
                    "SELECT {} FROM {}",
                    projection
                        .iter()
                        .map(|p| p.to_string())
                        .collect::<Vec<_>>()
                        .join(", "),
                    from
                )?;
                if let Some(where_clause) = where_clause {
                    write!(f, " WHERE {}", where_clause)?;
                }
                if !group_by.is_empty() {
                    write!(
                        f,
                        " GROUP BY {}",
                        group_by
                            .iter()
                            .map(|p| p.to_string())
                            .collect::<Vec<_>>()
                            .join(", "),
                    )?;
                }
                Ok(())
            }
        }
    }
}

impl Display for SelectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.body)?;
        if self.order_by.len() > 0 {
            write!(
                f,
                " ORDER BY {}",
                self.order_by
                    .iter()
                    .map(|o| o.to_string())
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }
        if let Some(limit) = &self.limit {
            write!(f, " LIMIT {}", limit)?;
        }
        if let Some(offset) = &self.offset {
            write!(f, " OFFSET {}", offset)?;
        }
        Ok(())
    }
}

impl Display for OrderByExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.expr)?;
        if let Some(asc) = &self.asc {
            if *asc {
                write!(f, " ASC")?;
            } else {
                write!(f, " DESC")?;
            }
        }
        Ok(())
    }
}
