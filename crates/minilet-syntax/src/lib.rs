mod span;
mod util;

pub mod expr;
pub mod literal;
pub mod op;
pub mod punctured;
pub mod relaxed;
pub mod spacing;
pub mod stmt;
pub mod stmts;
pub mod term;
pub mod token;
pub mod trivia;
pub mod unary_op;

use parcom::{metrics::LineColumn, ParseResult, ParseStream};

pub use expr::Expr;
pub use literal::Literal;
pub use spacing::Spacing;
pub use span::Span;
pub use stmt::Stmt;
pub use stmts::Stmts;
pub use term::{Ident, Term, Tuple, Unary};
pub use trivia::Trivia;

pub trait Parse: Sized {
    type Error;
    type Fatal;
    fn parse<S: InputStream>(
        input: S,
    ) -> impl std::future::Future<Output = ParseResult<S, Self, Self::Error, Self::Fatal>>;
}

pub trait InputStream: ParseStream<Segment = str, Metrics = LineColumn> {}
impl<S: ParseStream<Segment = str, Metrics = LineColumn>> InputStream for S {}
