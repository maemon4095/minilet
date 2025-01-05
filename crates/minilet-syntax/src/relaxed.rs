use crate::{InputStream, Parse, Trivia};
use parcom::prelude::*;

#[derive(Debug)]
pub struct Relaxed<T: Parse> {
    pub leading_trivia: Trivia,
    pub item: T,
    pub trailing_trivia: Trivia,
}

impl<T: Parse> Parse for Relaxed<T> {
    type Error = T::Error;
    type Fatal = T::Fatal;

    async fn parse<S: InputStream>(input: S) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (leading_trivia, rest) = match Trivia::parse(input).await {
            Done(v, r) => (v, r),
            _ => unreachable!(),
        };

        let (item, rest) = match T::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(e, r),
            Fatal(e, r) => return Fatal(e, r),
        };

        let (trailing_trivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            _ => unreachable!(),
        };

        let me = Self {
            leading_trivia,
            item,
            trailing_trivia,
        };

        Done(me, rest)
    }
}
