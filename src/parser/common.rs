use nom::multi::{separated_list0, separated_list1};
use nom::Slice;

use crate::ast::Ident;

use super::{
    error::PError,
    token::{Token, TokenKind},
    IResult, Input,
};

pub const MIN_PRECEDENCE: u32 = 0;

pub enum AffixKind {
    Prefix,
    Infix,
}

pub fn match_text(text: &'static str) -> impl FnMut(Input) -> IResult<&Token> {
    move |i| match i.get(0).filter(|token| token.text() == text) {
        Some(token) => Ok((i.slice(1..), token)),
        None => Err(nom::Err::Error(PError(format!(
            "text {text} does not match"
        )))),
    }
}

pub fn match_token(kind: TokenKind) -> impl FnMut(Input) -> IResult<&Token> {
    move |i| match i.get(0).filter(|token| token.kind == kind) {
        Some(token) => Ok((i.slice(1..), token)),
        None => Err(nom::Err::Error(PError(format!(
            "token kind {kind} does not match"
        )))),
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

pub fn comma_separated_list0<'a, T>(
    item: impl FnMut(Input<'a>) -> IResult<'a, T>,
) -> impl FnMut(Input<'a>) -> IResult<'a, Vec<T>> {
    separated_list0(match_text(","), item)
}

pub fn comma_separated_list1<'a, T>(
    item: impl FnMut(Input<'a>) -> IResult<'a, T>,
) -> impl FnMut(Input<'a>) -> IResult<'a, Vec<T>> {
    separated_list1(match_text(","), item)
}
