use crate::{
    punctured::{ParsePuncturedError, Punctured},
    stmt::Stmt,
    token::{ParseTokenError, Semi},
    Expr, Parse, Trivia,
};
use parcom::prelude::*;

#[derive(Debug)]
pub struct Stmts {
    pub stmts: Punctured<Stmt, StmtSeparator>,
    pub trailing_semi: Option<(Trivia, Semi)>,
}

impl Stmts {
    pub fn last_expr(&self) -> Option<&Expr> {
        if self.trailing_semi.is_some() {
            return None;
        }

        match self.stmts.last() {
            Some(Stmt::Expr(e)) => Some(&e),
            _ => None,
        }
    }
}

impl Parse for Stmts {
    type Error = Never;
    type Fatal = ParseStmtsError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (stmts, rest) = match Punctured::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, _) => return e.never(),
            Fatal(e, r) => return Fatal(ParseStmtsError::Punctured(e), r),
        };

        let anchor = rest.anchor();
        let (trivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (trailing_semi, rest) = match Semi::parse(rest).await {
            Done(v, r) => (Some((trivia, v)), r),
            Fail(_, r) => (None, r.rewind(anchor)),
            Fatal(e, _) => e.never(),
        };

        let me = Self {
            stmts,
            trailing_semi,
        };

        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseStmtsError {
    Punctured(ParsePuncturedError<Stmt, StmtSeparator>),
}

#[derive(Debug)]
pub struct StmtSeparator {
    pub leading_trivia: Trivia,
    pub semi: Semi,
    pub trailing_trivia: Trivia,
}

#[derive(Debug)]
pub struct ParseStmtSeparatorError {
    pub semi: ParseTokenError<Semi>,
}

impl Parse for StmtSeparator {
    type Error = ParseStmtSeparatorError;
    type Fatal = Never;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (leading_trivia, rest) = match Trivia::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (semi, rest) = match Semi::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(ParseStmtSeparatorError { semi: e }, r),
            Fatal(e, _) => e.never(),
        };

        let (trailing_trivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let me = Self {
            leading_trivia,
            semi,
            trailing_trivia,
        };

        Done(me, rest)
    }
}
