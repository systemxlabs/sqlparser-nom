pub mod common;
pub mod error;
pub mod expr;
pub mod set_expr;
pub mod statement;
pub mod token;

pub type Input<'a> = &'a [token::Token<'a>];
pub type IResult<'a, Output> = nom::IResult<Input<'a>, Output, error::PError>;

pub fn tokenize_sql(sql: &str) -> Vec<token::Token> {
    token::Tokenizer::new(sql).collect::<Vec<_>>()
}
