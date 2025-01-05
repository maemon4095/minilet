use crate::{
    expr::ParseExprError,
    spacing::ParseSpacingError,
    term::ident::ParseIdentError,
    token::{self, ParseTokenError},
    Expr, Ident, Parse, Spacing, Trivia,
};
use parcom::prelude::*;

#[derive(Debug)]
pub struct StmtLet {
    pub let_token: token::Let,
    pub let_spacing: Spacing,
    pub ident: Ident,
    pub ident_trivia: Trivia,
    pub eq: token::Eq,
    pub eq_trivia: Trivia,
    pub expr: Expr,
}

impl Parse for StmtLet {
    type Error = ParseStmtLetError;
    type Fatal = ParseStmtLetError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (let_token, rest) = match token::Let::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, r) => {
                return Fail(ParseStmtLetError::Let(e), r);
            }
            Fatal(e, _) => e.never(),
        };

        let (let_spacing, rest) = match Spacing::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => {
                return Fatal(ParseStmtLetError::Spacing(e), r);
            }
            Fatal(e, _) => e.never(),
        };

        let (ident, rest) = match Ident::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => {
                return Fatal(ParseStmtLetError::Ident(e), r);
            }
            Fatal(e, _) => e.never(),
        };

        let (ident_trivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (eq, rest) = match token::Eq::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => {
                return Fatal(ParseStmtLetError::Eq(e), r);
            }
            Fatal(e, _) => e.never(),
        };

        let (eq_trivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (expr, rest) = match Expr::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => {
                return Fatal(ParseStmtLetError::Expr(e), r);
            }
            Fatal(e, r) => return Fatal(ParseStmtLetError::Expr(e), r),
        };

        let me = Self {
            let_token,
            let_spacing,
            ident,
            ident_trivia,
            eq,
            eq_trivia,
            expr,
        };
        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseStmtLetError {
    Let(ParseTokenError<token::Let>),
    Spacing(ParseSpacingError),
    Eq(ParseTokenError<token::Eq>),
    Ident(ParseIdentError),
    Expr(ParseExprError),
}
