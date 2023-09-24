use crate::ast::expr::Expr;
use crate::ast::statement::SelectStatement;
use crate::ast::Ident;

#[derive(Debug, Clone)]
pub enum TableRef {
    BaseTable {
        name: TableName,
        alias: Option<Ident>,
    },
    Subquery {
        subquery: Box<SelectStatement>,
        alias: Option<Ident>,
    },
    Join {
        op: JoinOp,
        condition: JoinCondition,
        left: Box<TableRef>,
        right: Box<TableRef>,
    },
}
impl std::fmt::Display for TableRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TableRef::BaseTable { name, alias } => {
                write!(f, "{name}")?;
                if let Some(alias) = alias {
                    write!(f, " AS {alias}")?;
                }
                Ok(())
            }
            TableRef::Subquery { subquery, alias } => {
                write!(f, "({subquery})")?;
                if let Some(alias) = alias {
                    write!(f, " AS {alias}")?;
                }
                Ok(())
            }
            TableRef::Join {
                op,
                condition,
                left,
                right,
            } => {
                write!(f, "{left} {op} {right}")?;
                match condition {
                    JoinCondition::On(expr) => write!(f, " ON ({})", expr)?,
                    JoinCondition::None => {}
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct TableName {
    pub database: Option<Ident>,
    pub table: Ident,
}
impl std::fmt::Display for TableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(database) = self.database.as_ref() {
            write!(f, "{database}.")?;
        }
        write!(f, "{}", self.table)?;
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub enum JoinOp {
    Inner,
    LeftOuter,
    RightOuter,
    FullOuter,
    CrossJoin,
}
impl std::fmt::Display for JoinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JoinOp::Inner => write!(f, "INNER JOIN"),
            JoinOp::LeftOuter => write!(f, "LEFT OUTER JOIN"),
            JoinOp::RightOuter => write!(f, "RIGHT OUTER JOIN"),
            JoinOp::FullOuter => write!(f, "FULL OUTER JOIN"),
            JoinOp::CrossJoin => write!(f, "CROSS JOIN"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum JoinCondition {
    On(Box<Expr>),
    None,
}
