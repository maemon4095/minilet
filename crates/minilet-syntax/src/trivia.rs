use crate::util::any_char;
use crate::{Parse, Span};
use parcom::prelude::*;
use parcom::{Never, Parser};

#[derive(Debug)]
pub struct Trivia {
    pub text: String,
    pub span: Span,
}

impl Trivia {
    pub fn empty(span: Span) -> Self {
        Self {
            text: String::new(),
            span,
        }
    }
}

impl Parse for Trivia {
    type Error = Never;
    type Fatal = Never;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let start = input.metrics();

        let mut rest = input;
        let mut buf = String::new();
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
