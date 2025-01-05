use crate::{expr::ParseExprError, Expr, Parse};
use parcom::prelude::*;

#[derive(Debug)]
pub struct StmtExpr {
    pub expr: Expr,
}

impl Parse for StmtExpr {
    type Error = ParseStmtExprError;
    type Fatal = ParseStmtExprError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (expr, rest) = match Expr::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, r) => {
                return Fail(ParseStmtExprError::Expr(e), r);
            }
            Fatal(e, r) => return Fatal(ParseStmtExprError::Expr(e), r),
        };

        let me = Self { expr };
        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseStmtExprError {
    Expr(ParseExprError),
}
