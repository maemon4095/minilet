pub mod app;
pub mod block;
pub mod ident;
pub mod tuple;
pub mod unary;

use crate::{
    literal::{Literal, ParseLiteralError},
    InputStream, Parse,
};
use app::App;
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

// todo: add fn call

#[derive(Debug)]
pub enum Term {
    Tuple(Tuple),
    Literal(Literal),
    Ident(Ident),
    Unary(Box<Unary>),
    Block(Block),
    App(Box<App>),
}

impl Parse for Term {
    type Error = ParseTermError;
    type Fatal = ParseTermError;

    fn parse<S: InputStream>(
        input: S,
    ) -> impl Future<Output = ParseResult<S, Self, <Self as Parse>::Error, <Self as Parse>::Fatal>>
    {
        Box::pin(async {
            let anchor = input.anchor();
            let input = match Unary::parse(input).await {
                Done(v, r) => {
                    let me = Self::Unary(Box::new(v));
                    return Done(me, r);
                }
                Fail(_, r) => r.rewind(anchor),
                Fatal(e, r) => return Fatal(ParseTermError::Unary(Box::new(e)), r),
            };

            let (mut receiver, mut rest) = match parse_atom(input).await {
                Done(v, r) => (v, r),
                e @ _ => return e,
            };

            loop {
                let anchor = rest.anchor();
                match Tuple::parse(rest).await {
                    Done(arg, r) => {
                        rest = r;
                        receiver = Term::App(Box::new(App { receiver, arg }))
                    }
                    Fail(_, r) => {
                        rest = r.rewind(anchor);
                        break;
                    }
                    Fatal(e, r) => return Fatal(ParseTermError::Tuple(e), r),
                }
            }

            Done(receiver, rest)
        })
    }
}

async fn parse_atom<S: InputStream>(
    input: S,
) -> ParseResult<S, Term, ParseTermError, ParseTermError> {
    let anchor = input.anchor();
    let input = match Block::parse(input).await {
        Done(v, r) => {
            let me = Term::Block(v);
            return Done(me, r);
        }

        Fail(_, r) => r.rewind(anchor),
        Fatal(e, r) => {
            return Fatal(ParseTermError::Block(e), r);
        }
    };

    let anchor = input.anchor();
    let input = match Tuple::parse(input).await {
        Done(v, r) => {
            let me = Term::Tuple(v);
            return Done(me, r);
        }

        Fail(_, r) => r.rewind(anchor),
        Fatal(e, r) => {
            return Fatal(ParseTermError::Tuple(e), r);
        }
    };

    let anchor = input.anchor();
    let input = match Ident::parse(input).await {
        Done(v, r) => {
            let me = Term::Ident(v);
            return Done(me, r);
        }
        Fail(_, r) => r.rewind(anchor),
        Fatal(e, _) => e.never(),
    };

    match Literal::parse(input).await {
        Done(v, r) => {
            let me = Term::Literal(v);
            Done(me, r)
        }
        Fail(e, r) => Fail(ParseTermError::Literal(e), r),
        Fatal(e, r) => Fatal(ParseTermError::Literal(e), r),
    }
}

#[derive(Debug)]
pub enum ParseTermError {
    Unary(Box<ParseUnaryError>),
    Tuple(ParseTupleError),
    Literal(ParseLiteralError),
    Block(ParseBlockError),
}
