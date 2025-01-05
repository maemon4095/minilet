use crate::util::any_char;
use crate::{Parse, Span};
use parcom::prelude::*;
use parcom::{Never, Parser};

#[derive(Debug)]
pub struct Spacing {
    pub text: String,
    pub span: Span,
}

impl Parse for Spacing {
    type Error = ParseSpacingError;
    type Fatal = Never;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let start = input.metrics();
        let mut buf = String::new();
        let mut rest = match any_char().parse(input).await {
            Done(c, r) if c.is_ascii_whitespace() => {
                buf.push(c);
                r
            }
            Done(_, r) => return Fail(ParseSpacingError::MissingSpace, r.into()),
            Fail(_, r) => return Fail(ParseSpacingError::MissingSpace, r),
            Fatal(e, _) => e.never(),
        };

        let mut anchor = rest.anchor();
        loop {
            match any_char().parse(rest).await {
                Done(c, r) if c.is_ascii_whitespace() => {
                    buf.push(c);
                    anchor = r.anchor();
                    rest = r;
                }
                Done(_, r) => {
                    rest = r.rewind(anchor);
                    break;
                }
                Fail(_, r) | Fatal(_, r) => {
                    rest = r.rewind(anchor);
                    break;
                }
            };
        }

        let end = rest.metrics();
        let me = Self {
            text: buf,
            span: Span::new(start, end),
        };
        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseSpacingError {
    MissingSpace,
}
