mod bind;

use crate::{expr::ParseExprError, Expr, Parse};
use bind::{ParseStmtLetError, StmtLet};
use parcom::prelude::*;

#[derive(Debug)]
pub enum Stmt {
    Let(StmtLet),
    Expr(Expr),
}

impl Parse for Stmt {
    type Error = ParseStmtError;
    type Fatal = ParseStmtError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let anchor = input.anchor();
        let input = match StmtLet::parse(input).await {
            Done(v, r) => {
                return Done(Stmt::Let(v), r);
            }
            Fail(_, r) => r.rewind(anchor),
            Fatal(e, r) => {
                return Fatal(ParseStmtError::Let(e), r);
            }
        };

        match Expr::parse(input).await {
            Done(v, r) => Done(Stmt::Expr(v), r),
            Fail(e, r) => Fail(ParseStmtError::Expr(e), r),
            Fatal(e, r) => Fatal(ParseStmtError::Expr(e), r),
        }
    }
}

#[derive(Debug)]
pub enum ParseStmtError {
    Expr(ParseExprError),
    Let(ParseStmtLetError),
}
