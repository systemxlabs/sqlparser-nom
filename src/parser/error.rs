use super::{IResult, Input};

#[derive(Debug)]
pub struct PError(String);
impl PError {
    pub fn from<O>(message: &str) -> IResult<O> {
        Err(nom::Err::Error(PError(message.to_string())))
    }
}

impl nom::error::ParseError<Input<'_>> for PError {
    fn from_error_kind(input: Input<'_>, kind: nom::error::ErrorKind) -> Self {
        PError("parse error".to_string())
    }

    fn append(input: Input<'_>, kind: nom::error::ErrorKind, other: Self) -> Self {
        other
    }
}
