use crate::{util::any_char, Parse, Span};
use parcom::prelude::*;

#[derive(Debug)]
pub struct Ident {
    pub text: String,
    pub span: Span,
}

impl Parse for Ident {
    type Error = ParseIdentError;
    type Fatal = Never;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let start = input.metrics();
        let mut buf = String::new();

        let mut rest = input;
        let rest = loop {
            let anchor = rest.anchor();
            match any_char().parse(rest).await {
                Done(c, r) => {
                    if buf.is_empty() {
                        if !c.is_ascii_alphabetic() {
                            break r;
                        }
                    } else {
                        if !c.is_ascii_alphanumeric() {
                            break r;
                        }
                    }

                    buf.push(c);
                    rest = r;
                }
                Fail(_, r) => break r.rewind(anchor),
                Fatal(_, _) => unreachable!(),
            }
        };

        if buf.is_empty() {
            Fail(
                ParseIdentError::Missing {
                    span: Span::points(start),
                },
                rest.into(),
            )
        } else {
            let end = rest.metrics();

            let me = Self {
                text: buf,
                span: Span::new(start, end),
            };
            Done(me, rest)
        }
    }
}

#[derive(Debug)]
pub enum ParseIdentError {
    Missing { span: Span },
}
