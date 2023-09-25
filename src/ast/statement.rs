use super::{expr::Expr, set_expr::SetExpr};
use crate::ast::Ident;

#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub with: Option<With>,
    pub body: SetExpr,
    pub order_by: Vec<OrderByExpr>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
}
impl std::fmt::Display for SelectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(with) = &self.with {
            write!(f, "{} ", with)?;
        }
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

#[derive(Debug, Clone)]
pub struct OrderByExpr {
    pub expr: Expr,
    pub asc: Option<bool>,
}
impl std::fmt::Display for OrderByExpr {
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

#[derive(Debug, Clone)]
pub struct With {
    pub recursive: bool,
    pub cte_tables: Vec<Cte>,
}
impl std::fmt::Display for With {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "WITH {}{}",
            if self.recursive { "RECURSIVE " } else { "" },
            self.cte_tables
                .iter()
                .map(|cte| cte.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub struct Cte {
    pub alias: TableAlias,
    pub query: Box<SelectStatement>,
}
impl std::fmt::Display for Cte {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} AS ({})", self.alias, self.query)
    }
}

#[derive(Debug, Clone)]
pub struct TableAlias {
    pub name: Ident,
    pub columns: Vec<Ident>,
}
impl std::fmt::Display for TableAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)?;
        if !self.columns.is_empty() {
            write!(
                f,
                " ({})",
                self.columns
                    .iter()
                    .map(|col| col.to_string())
                    .collect::<Vec<String>>()
                    .join(", ")
            )?;
        }
        Ok(())
    }
}
