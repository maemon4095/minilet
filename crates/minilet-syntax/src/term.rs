pub mod block;
pub mod ident;
pub mod parenthesized;
pub mod unary;

use crate::{
    literal::{Literal, ParseLiteralError},
    Parse,
};
use block::{Block, ParseBlockError};
use parcom::{
    ParseResult::{Done, Fail, Fatal},
    ShouldNeverExtension,
};
use parenthesized::ParseParenthesizedError;
use std::future::Future;
use unary::ParseUnaryError;

pub use ident::Ident;
pub use parenthesized::Parenthesized;
pub use unary::Unary;

#[derive(Debug)]
pub enum Term {
    Parenthesized(Parenthesized),
    Literal(Literal),
    Ident(Ident),
    Unary(Box<Unary>),
    Block(Block),
}

impl Parse for Term {
    type Error = ParseTermError;
    type Fatal = ParseTermError;

    fn parse<S: crate::InputStream>(
        input: S,
    ) -> impl Future<Output = parcom::ParseResult<S, Self, <Self as Parse>::Error, <Self as Parse>::Fatal>>
    {
        Box::pin(async {
            let anchor = input.anchor();
            let input = match Block::parse(input).await {
                Done(v, r) => {
                    let me = Self::Block(v);
                    return Done(me, r);
                }

                Fail(_, r) => r.rewind(anchor),
                Fatal(e, r) => {
                    return Fatal(ParseTermError::Block(e), r);
                }
            };

            let anchor = input.anchor();
            let input = match Unary::parse(input).await {
                Done(v, r) => {
                    let me = Self::Unary(Box::new(v));
                    return Done(me, r);
                }
                Fail(_, r) => r.rewind(anchor),
                Fatal(e, r) => return Fatal(ParseTermError::Unary(Box::new(e)), r),
            };

            let anchor = input.anchor();
            let input = match Parenthesized::parse(input).await {
                Done(v, r) => {
                    let me = Self::Parenthesized(v);
                    return Done(me, r);
                }

                Fail(_, r) => r.rewind(anchor),
                Fatal(e, r) => {
                    return Fatal(ParseTermError::Parenthesized(e), r);
                }
            };

            let anchor = input.anchor();
            let input = match Ident::parse(input).await {
                Done(v, r) => {
                    let me = Self::Ident(v);
                    return Done(me, r);
                }
                Fail(_, r) => r.rewind(anchor),
                Fatal(e, _) => e.never(),
            };

            match Literal::parse(input).await {
                Done(v, r) => {
                    let me = Self::Literal(v);
                    Done(me, r)
                }
                Fail(e, r) => Fail(ParseTermError::Literal(e), r),
                Fatal(e, r) => Fatal(ParseTermError::Literal(e), r),
            }
        })
    }
}

#[derive(Debug)]
pub enum ParseTermError {
    Unary(Box<ParseUnaryError>),
    Parenthesized(ParseParenthesizedError),
    Literal(ParseLiteralError),
    Block(ParseBlockError),
}
