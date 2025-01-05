use std::ops::Deref;

use parcom::prelude::*;
use parcom::ParcomStream;

pub fn any_char() -> AnyChar {
    AnyChar
}

pub struct AnyChar;

impl<S: ParcomStream<Segment = str>> Parser<S> for AnyChar {
    type Output = char;
    type Error = ();
    type Fault = Never;

    async fn parse(&self, input: S) -> ParserResult<S, Self> {
        let mut segments = input.segments();

        loop {
            let Some(segment) = segments.next(0).await else {
                break Fail((), input.into());
            };
            let segment = segment.deref();

            if let Some(c) = segment.chars().next() {
                let rest = input.advance(c.len_utf8().into()).await;
                break Done(c, rest);
            }
        }
    }
}
