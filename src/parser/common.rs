use nom::Slice;

use crate::ast::Ident;

use super::{
    error::PError,
    token::{Token, TokenKind},
    IResult, Input,
};

pub fn match_token(kind: TokenKind) -> impl FnMut(Input) -> IResult<&Token> {
    move |i| match i.get(0).filter(|token| token.kind == kind) {
        Some(token) => Ok((i.slice(1..), token)),
        None => PError::from("token kind does not match"),
    }
}

pub fn ident(i: Input) -> IResult<Ident> {
    match i.get(0).filter(|token| !token.kind.is_keyword()) {
        Some(token) => Ok((
            i.slice(1..),
            Ident {
                value: token.text().to_string(),
            },
        )),
        None => PError::from("ident should not be a keyword"),
    }
}

#[test]
pub fn test() {
    use crate::parser::tokenize_sql;
    let tokens = tokenize_sql("a");
    println!("{:?}", ident(&tokens));
}
