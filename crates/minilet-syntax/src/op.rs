use crate::spacing::Spacing;
use crate::token::Token;
use crate::util::any_char;
use crate::{token, Parse, Span};
use parcom::parsers::binary_expr::Operator;
use parcom::prelude::*;

#[derive(Debug)]
pub enum Op {
    Add {
        leading_spacing: Spacing,
        token: token::Plus,
        trailing_spacing: Spacing,
    },
    Sub {
        leading_spacing: Spacing,
        token: token::Minus,
        trailing_spacing: Spacing,
    },
    Mul {
        leading_spacing: Spacing,
        token: token::Asterisk,
        trailing_spacing: Spacing,
    },
    Div {
        leading_spacing: Spacing,
        token: token::Slash,
        trailing_spacing: Spacing,
    },
}

impl Operator for Op {
    fn precedence(&self) -> usize {
        match self {
            Op::Add { .. } => 1,
            Op::Sub { .. } => 1,
            Op::Mul { .. } => 2,
            Op::Div { .. } => 2,
        }
    }

    fn associativity(&self) -> parcom::parsers::binary_expr::Associativity {
        parcom::parsers::binary_expr::Associativity::Left
    }
}

impl Parse for Op {
    type Error = ParseOpError;
    type Fatal = Never;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let start = input.metrics();

        let (leading_spacing, rest) = match Spacing::parse(input).await {
            Done(v, r) => (v, r),
            Fail(_, r) => {
                return Fail(ParseOpError::MissingLeadingSpace(Span::points(start)), r);
            }
            Fatal(e, _) => e.never(),
        };

        let just_op = rest.metrics();
        let span = Span::points(just_op);
        let (op_char, rest) = match any_char().parse(rest).await {
            Done(v, r) => (v, r),
            Fail(_, r) => return Fail(ParseOpError::NoSymbol(span), r),
            Fatal(e, _) => e.never(),
        };

        let before_trailing_space = rest.metrics();
        let (trailing_spacing, rest) = match Spacing::parse(rest).await {
            Done(v, r) => (v, r),
            Fail(_, r) => {
                return Fail(
                    ParseOpError::MissingTrailingSpace(Span::points(before_trailing_space)),
                    r,
                );
            }
            Fatal(e, _) => e.never(),
        };

        let op = match op_char {
            token::Plus::TOKEN => Op::Add {
                leading_spacing,
                token: token::Plus::from_span(span),
                trailing_spacing,
            },
            token::Minus::TOKEN => Op::Sub {
                leading_spacing,
                token: token::Minus::from_span(span),
                trailing_spacing,
            },
            token::Asterisk::TOKEN => Op::Mul {
                leading_spacing,
                token: token::Asterisk::from_span(span),
                trailing_spacing,
            },
            token::Slash::TOKEN => Op::Div {
                leading_spacing,
                token: token::Slash::from_span(span),
                trailing_spacing,
            },
            _ => {
                return Fail(
                    ParseOpError::UnknownSymbol(Span::points(start)),
                    rest.into(),
                )
            }
        };
        Done(op, rest)
    }
}
#[derive(Debug)]
pub enum ParseOpError {
    MissingLeadingSpace(Span),
    MissingTrailingSpace(Span),
    NoSymbol(Span),
    UnknownSymbol(Span),
}
