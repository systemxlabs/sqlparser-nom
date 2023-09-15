#[derive(Debug)]
pub enum Statement {
    CreateTable(CreateTableStatement),
}

/// CREATE TABLE
#[derive(Debug)]
pub struct CreateTableStatement {
    pub name: Ident,
    pub columns: Vec<ColumnDef>,
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
    /// Boolean
    Boolean,

    /// Tiny integer with optional display width e.g. TINYINT or TINYINT(3)
    TinyInt(Option<u64>),
}