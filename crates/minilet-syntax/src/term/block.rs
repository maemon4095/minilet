use crate::{
    stmts::{ParseStmtsError, Stmts},
    token::{LBrace, ParseTokenError, RBrace},
    Parse, Trivia,
};
use parcom::prelude::*;

#[derive(Debug)]
pub struct Block {
    pub lbrace: LBrace,
    pub ltrivia: Trivia,
    pub stmts: Stmts,
    pub rtrivia: Trivia,
    pub rbrace: RBrace,
}

impl Parse for Block {
    type Error = ParseBlockError;
    type Fatal = ParseBlockError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let (lbrace, rest) = match LBrace::parse(input).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(ParseBlockError::MissingOpeningBrace(e), r),
            Fatal(e, _) => e.never(),
        };

        let (ltrivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (stmts, rest) = match Stmts::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) => e.never(),
            Fatal(e, r) => return Fatal(ParseBlockError::Stmts(e), r),
        };

        let (rtrivia, rest) = match Trivia::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, _) | Fatal(e, _) => e.never(),
        };

        let (rbrace, rest) = match RBrace::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(e, r) => return Fail(ParseBlockError::MissingClosingBrace(e), r),
            Fatal(e, _) => e.never(),
        };

        let me = Self {
            lbrace,
            ltrivia,
            stmts,
            rtrivia,
            rbrace,
        };

        Done(me, rest)
    }
}

#[derive(Debug)]
pub enum ParseBlockError {
    MissingOpeningBrace(ParseTokenError<LBrace>),
    Stmts(ParseStmtsError),
    MissingClosingBrace(ParseTokenError<RBrace>),
}
