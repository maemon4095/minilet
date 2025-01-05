use crate::{
    expr::Expr,
    punctured::{ParsePuncturedError, Punctured},
    relaxed::Relaxed,
    token::{self, Comma},
    InputStream, Parse, Span, Trivia,
};
use parcom::{
    ParseResult::{Done, Fail, Fatal},
    ShouldNeverExtension,
};
#[derive(Debug)]
pub struct Tuple {
    pub lparen: token::LParen,
    pub ltrivia: Trivia,
    pub items: Punctured<Expr, Relaxed<token::Comma>>,
    pub rtrivia: Trivia,
    pub rparen: token::RParen,
}

impl Parse for Tuple {
    type Error = ParseTupleError;
    type Fatal = ParseTupleError;

    async fn parse<S: InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (lparen, rest) = match token::LParen::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(ParseTupleError::MissingOpeningParen(e.span), r),
            Fatal(e, _) => e.never(),
        };

        let (ltrivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (items, rest) = match Punctured::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) => e.never(),
            Fatal(e, r) => return Fatal(ParseTupleError::Punct(e), r),
        };

        let (rtrivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (rparen, rest) = match token::RParen::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fatal(ParseTupleError::MissingOpeningParen(e.span), r),
            Fatal(e, _) => e.never(),
        };

        let me = Tuple {
            lparen,
            ltrivia,
            items,
            rtrivia,
            rparen,
        };
        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseTupleError {
    MissingOpeningParen(Span),
    MissingClosingParen(Span),
    Punct(ParsePuncturedError<Expr, Relaxed<Comma>>),
}
