mod display;

#[derive(Debug)]
pub enum Statement {
    CreateTable(CreateTableStatement),
    Select(SelectStatement),
}

/// CREATE TABLE
#[derive(Debug)]
pub struct CreateTableStatement {
    pub name: Ident,
    pub columns: Vec<ColumnDef>,
}

#[derive(Debug)]
pub struct SelectStatement {
    pub body: SetExpr,
    pub limit: Option<Expr>,
    pub offset: Option<Expr>,
}

#[derive(Debug)]
pub enum SetExpr {
    Select(Select),
}

#[derive(Debug)]
pub struct Select {
    pub projection: Vec<SelectItem>,
    pub from: Ident,
    pub where_clause: Expr,
}

#[derive(Debug)]
pub enum SelectItem {
    UnnamedExpr(Expr),
    ExprWithAlias { expr: Expr, alias: Ident },
    Wildcard,
}

#[derive(Debug)]
pub enum Expr {
    /// Identifier e.g. table name or column name
    Identifier(Ident),
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
}

/// Literal values
#[derive(Debug)]
pub enum Literal {
    String(String),
    UnsignedInteger(usize),
    UnsignedFloat(f64),
}

/// An identifier
#[derive(derive_new::new, Debug)]
pub struct Ident {
    /// The value of the identifier without quotes.
    pub value: String,
}

/// Representation of a new column to define in a table
#[derive(Debug)]
pub struct ColumnDef {
    pub name: Ident,
    pub column_type: DataType,
    pub constraints: Vec<ColumnConstraint>,
}

#[derive(Debug)]
pub struct ColumnConstraint {
    pub name: Option<Ident>,
    pub constraint: ColumnConstraintKind,
}

#[derive(Debug)]
pub enum ColumnConstraintKind {
    PrimaryKey,
    NotNull,
}

/// SQL data types
#[derive(Debug)]
pub enum DataType {
    Boolean,
    TinyInt(Option<usize>),
    SmallInt(Option<usize>),
    Integer(Option<usize>),
    BigInt(Option<usize>),
    Varchar(Option<usize>),
}
