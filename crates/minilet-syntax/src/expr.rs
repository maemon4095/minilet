use crate::term::ParseTermError;
use crate::{term::Term, Parse};
use parcom::parsers::binary_expr::BinaryExprParser;
use parcom::prelude::*;

use crate::op::Op;

#[derive(Debug)]
pub enum Expr {
    Term(Box<Term>),
    Bin(Box<BinOp>),
}

impl Parse for Expr {
    type Error = ParseExprError;
    type Fatal = ParseExprError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let result = BinaryExprParser::new(Term::parse, Op::parse)
            .parse(input)
            .await;

        match result {
            Done((v, _), r) => Done(v, r),
            Fail(e, r) => Fail(ParseExprError { term: Box::new(e) }, r),
            Fatal(e, r) => Fatal(
                ParseExprError {
                    term: Box::new(e.always_last()),
                },
                r,
            ),
        }
    }
}

impl From<(Expr, Op, Expr)> for Expr {
    fn from((lhs, op, rhs): (Expr, Op, Expr)) -> Self {
        Expr::Bin(Box::new(BinOp { lhs, op, rhs }))
    }
}

impl From<Term> for Expr {
    fn from(value: Term) -> Self {
        Expr::Term(Box::new(value))
    }
}

#[derive(Debug)]
pub struct BinOp {
    pub lhs: Expr,
    pub op: Op,
    pub rhs: Expr,
}

#[derive(Debug)]
pub struct ParseExprError {
    pub term: Box<ParseTermError>,
}
