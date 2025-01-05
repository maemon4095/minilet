use crate::{util::any_char, Parse, Span};
use parcom::prelude::*;
use parcom::{Never, Parser};

#[derive(Debug)]
pub enum UnaryOp {
    Plus { span: Span },
    Sub { span: Span },
}

impl Parse for UnaryOp {
    type Error = ParseUnaryOpError;
    type Fatal = Never;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let start = input.metrics();
        let (op_char, rest) = match any_char().parse(input).await {
            Done(v, r) => (v, r),
            Fail(_, r) => {
                return Fail(
                    ParseUnaryOpError::Missing {
                        span: Span::points(start),
                    },
                    r,
                )
            }
            Fatal(e, _) => e.never(),
        };

        let end = rest.metrics();
        let span = Span::new(start, end);
        let op = match op_char {
            '+' => UnaryOp::Plus { span },
            '-' => UnaryOp::Sub { span },
            _ => return Fail(ParseUnaryOpError::UnknownSymbol { span }, rest.into()),
        };

        Done(op, rest)
    }
}

#[derive(Debug)]
pub enum ParseUnaryOpError {
    Missing { span: Span },
    UnknownSymbol { span: Span },
}
