use crate::{
    expr::{Expr, ParseExprError},
    token, Parse, Span, Trivia,
};
use parcom::{
    ParseResult::{Done, Fail, Fatal},
    ShouldNeverExtension,
};
#[derive(Debug)]
pub struct Parenthesized {
    pub lparen: token::LParen,
    pub ltrivia: Trivia,
    pub expr: Expr,
    pub rtrivia: Trivia,
    pub rparen: token::RParen,
}

impl Parse for Parenthesized {
    type Error = ParseParenthesizedError;
    type Fatal = ParseParenthesizedError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (lparen, rest) = match token::LParen::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(ParseParenthesizedError::MissingOpeningParen(e.span), r),
            Fatal(e, _) => e.never(),
        };

        let (ltrivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (expr, rest) = match Expr::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fatal(ParseParenthesizedError::Expr(e), r),
            Fatal(e, r) => return Fatal(ParseParenthesizedError::Expr(e), r),
        };

        let (rtrivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (rparen, rest) = match token::RParen::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fatal(ParseParenthesizedError::MissingOpeningParen(e.span), r),
            Fatal(e, _) => e.never(),
        };

        let me = Parenthesized {
            lparen,
            ltrivia,
            expr,
            rtrivia,
            rparen,
        };
        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseParenthesizedError {
    MissingOpeningParen(Span),
    MissingClosingParen(Span),
    Expr(ParseExprError),
}
