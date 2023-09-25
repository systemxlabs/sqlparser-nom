use crate::ast::statement::SelectStatement;
use crate::parser::error::PError;
use crate::parser::statement::select_stmt;
use crate::parser::tokenize_sql;

pub mod ast;
pub mod parser;

pub fn parse_query(sql: &str) -> Result<SelectStatement, PError> {
    let tokens = tokenize_sql(sql);
    match select_stmt(&tokens) {
        Ok((_, stmt)) => Ok(stmt),
        Err(e) => match e {
            nom::Err::Error(e) => Err(e),
            nom::Err::Failure(e) => Err(e),
            _ => Err(PError("Failed to parse query sql".to_string())),
        },
    }
}
