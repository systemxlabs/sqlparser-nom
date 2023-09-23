use super::{expr::Expr, set_expr::SetExpr};

#[derive(Debug, Clone)]
pub struct SelectStatement {
    pub body: SetExpr,
    pub order_by: Vec<OrderByExpr>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
}
impl std::fmt::Display for SelectStatement {
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
