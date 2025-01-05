mod span;
mod util;

pub mod expr;
pub mod literal;
pub mod op;
pub mod punctured;
pub mod spacing;
pub mod stmt;
pub mod stmts;
pub mod term;
pub mod token;
pub mod trivia;
pub mod unary_op;

use std::marker::PhantomData;

use parcom::{metrics::LineColumn, ParseResult, ParseStream, Parser};

pub use expr::Expr;
pub use literal::Literal;
pub use spacing::Spacing;
pub use span::Span;
pub use term::{Ident, Parenthesized, Term, Unary};
pub use trivia::Trivia;

pub struct MiniLetExprParser {
    mark: PhantomData<()>,
}

impl<S: InputStream> Parser<S> for MiniLetExprParser {
    type Output = Expr;
    type Error = expr::ParseExprError;
    type Fault = expr::ParseExprError;

    fn parse(
        &self,
        input: S,
    ) -> impl std::future::Future<Output = ParseResult<S, Self::Output, Self::Error, Self::Fault>>
    {
        Expr::parse(input)
    }
}

pub trait Parse: Sized {
    type Error;
    type Fatal;
    fn parse<S: InputStream>(
        input: S,
    ) -> impl std::future::Future<Output = ParseResult<S, Self, Self::Error, Self::Fatal>>;
}

pub trait InputStream: ParseStream<Segment = str, Metrics = LineColumn> {}
impl<S: ParseStream<Segment = str, Metrics = LineColumn>> InputStream for S {}
