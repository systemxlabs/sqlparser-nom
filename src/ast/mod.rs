mod display;

#[derive(Debug)]
pub struct SelectStatement {
    pub body: SetExpr,
    pub order_by: Vec<OrderByExpr>,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
}

#[derive(Debug)]
pub struct OrderByExpr {
    pub expr: Expr,
    pub asc: Option<bool>,
}

#[derive(Debug)]
pub enum SetExpr {
    Select {
        projection: Vec<SelectItem>,
        from: Ident,
        where_clause: Option<Expr>,
        group_by: Vec<Expr>,
    },
}

#[derive(Debug)]
pub enum SelectItem {
    UnnamedExpr(Expr),
    ExprWithAlias { expr: Expr, alias: Ident },
    Wildcard,
}

#[derive(Debug)]
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

/// Unary operators
#[derive(Debug)]
pub enum UnaryOp {
    Plus,
    Minus,
}

/// Binary operators
#[derive(Debug)]
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

/// Literal values
#[derive(Debug)]
pub enum Literal {
    String(String),
    UnsignedInteger(usize),
    UnsignedFloat(f64),
}

/// An object name. e.g. database.table.column
pub struct ObjectName(Vec<Ident>);

/// An identifier
#[derive(derive_new::new, Debug, PartialEq, Eq)]
pub struct Ident {
    /// The value of the identifier without quotes.
    pub value: String,
}
