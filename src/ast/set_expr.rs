use super::{expr::Expr, Ident};

#[derive(Debug)]
pub enum SetExpr {
    Select {
        projection: Vec<SelectItem>,
        from: Ident,
        selection: Option<Expr>,
        group_by: Vec<Expr>,
        having: Option<Expr>,
    },
}
impl std::fmt::Display for SetExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Select {
                projection,
                from,
                selection,
                group_by,
                having,
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
                if let Some(selection) = selection {
                    write!(f, " WHERE {}", selection)?;
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
                if let Some(expr) = having {
                    write!(f, " Having {}", expr)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub enum SelectItem {
    UnnamedExpr(Expr),
    ExprWithAlias { expr: Expr, alias: Ident },
    Wildcard,
}
impl std::fmt::Display for SelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnnamedExpr(expr) => write!(f, "{}", expr),
            Self::ExprWithAlias { expr, alias } => write!(f, "{} AS {}", expr, alias),
            Self::Wildcard => write!(f, "*"),
        }
    }
}
