use super::{ParseTermError, Term};
use crate::{
    unary_op::{ParseUnaryOpError, UnaryOp},
    Parse,
};
use parcom::prelude::*;

#[derive(Debug)]
pub struct Unary {
    pub op: UnaryOp,
    pub term: Term,
}

impl Parse for Unary {
    type Error = ParseUnaryError;
    type Fatal = ParseUnaryError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (op, rest) = match UnaryOp::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(ParseUnaryError::Op(e), r),
            Fatal(e, _) => e.never(),
        };

        let (term, rest) = match Term::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(ParseUnaryError::Term(e), r),
            Fatal(e, r) => return Fatal(ParseUnaryError::Term(e), r),
        };

        let me = Self { op, term };
        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseUnaryError {
    Op(ParseUnaryOpError),
    Term(ParseTermError),
}
