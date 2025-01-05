pub mod integer;
pub mod string;

pub use integer::IntegerLiteral;
use integer::ParseIntegerLiteralError;
use parcom::ParseResult;
use string::ParseStringLiteralError;
pub use string::StringLiteral;

use crate::{Parse, Span};
use parcom::prelude::*;

#[derive(Debug)]
pub enum Literal {
    Integer(IntegerLiteral),
    String(StringLiteral),
}

impl Parse for Literal {
    type Error = ParseLiteralError;
    type Fatal = ParseLiteralError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let anchor = input.anchor();

        let (e0, input) = match IntegerLiteral::parse(input).await {
            Done(v, r) => return Done(Literal::Integer(v), r),
            Fail(e, r) => (e, r.rewind(anchor)),
            Fatal(e, _) => e.never(),
        };

        match StringLiteral::parse(input).await {
            Done(v, r) => return Done(Literal::String(v), r),
            Fail(e, r) => Fail(
                ParseLiteralError {
                    integer: e0,
                    string: e,
                },
                r,
            ),
            Fatal(e, r) => Fatal(
                ParseLiteralError {
                    integer: e0,
                    string: e,
                },
                r,
            ),
        }
    }
}

impl Literal {
    pub fn span(&self) -> Span {
        match self {
            Literal::Integer(v) => v.span.clone(),
            Literal::String(v) => v.span.clone(),
        }
    }
}

#[derive(Debug)]
pub struct ParseLiteralError {
    pub integer: ParseIntegerLiteralError,
    pub string: ParseStringLiteralError,
}
