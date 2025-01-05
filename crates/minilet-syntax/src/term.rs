pub mod block;
pub mod ident;
pub mod tuple;
pub mod unary;

use crate::{
    literal::{Literal, ParseLiteralError},
    Parse,
};
use block::{Block, ParseBlockError};
use parcom::{
    ParseResult::{self, Done, Fail, Fatal},
    ShouldNeverExtension,
};
use std::future::Future;
use tuple::ParseTupleError;
use unary::ParseUnaryError;

pub use ident::Ident;
pub use tuple::Tuple;
pub use unary::Unary;

#[derive(Debug)]
pub enum Term {
    Tuple(Tuple),
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
    ) -> impl Future<Output = ParseResult<S, Self, <Self as Parse>::Error, <Self as Parse>::Fatal>>
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
            let input = match Tuple::parse(input).await {
                Done(v, r) => {
                    let me = Self::Tuple(v);
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
    Parenthesized(ParseTupleError),
    Literal(ParseLiteralError),
    Block(ParseBlockError),
}
