use super::{
    BinaryOp, ColumnConstraint, ColumnConstraintKind, ColumnDef, CreateTableStatement, DataType,
    Expr, Ident, Literal, OrderByExpr, SelectItem, SelectStatement, SetExpr, Statement, UnaryOp,
};
use std::fmt::Display;

fn format_type_with_optional_length(
    f: &mut std::fmt::Formatter,
    sql_type: &'static str,
    len: &Option<usize>,
) -> std::fmt::Result {
    write!(f, "{sql_type}")?;
    if let Some(len) = len {
        write!(f, "({len})")?;
    }
    Ok(())
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Boolean => write!(f, "BOOLEAN"),
            Self::TinyInt(len) => format_type_with_optional_length(f, "TINYINT", len),
            Self::SmallInt(len) => format_type_with_optional_length(f, "SMALLINT", len),
            Self::Integer(len) => format_type_with_optional_length(f, "INTEGER", len),
            Self::BigInt(len) => format_type_with_optional_length(f, "BIGINT", len),
            Self::Varchar(len) => format_type_with_optional_length(f, "VARCHAR", len),
        }
    }
}

impl Display for ColumnConstraintKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PrimaryKey => write!(f, "PRIMARY KEY"),
            Self::NotNull => write!(f, "NOT NULL"),
        }
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{0}", self.value)
    }
}

impl Display for ColumnConstraint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.name {
            None => write!(f, "{}", self.constraint),
            Some(name) => write!(f, "CONSTRAINT {} {}", name, self.constraint),
        }
    }
}

impl Display for ColumnDef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.name, self.column_type)?;
        if !self.constraints.is_empty() {
            write!(
                f,
                " {}",
                self.constraints
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join(" ")
            )?;
        }
        Ok(())
    }
}

impl Display for CreateTableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CREATE TABLE {} ({});",
            self.name,
            self.columns
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CreateTable(s) => write!(f, "{}", s),
            Self::Select(s) => write!(f, "{}", s),
        }
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
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Identifier(ident) => write!(f, "{}", ident),
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
