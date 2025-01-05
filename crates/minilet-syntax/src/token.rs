use crate::{util::any_char, Parse, Span};
use parcom::parsers::primitive::str::atom;
use parcom::prelude::*;
use std::marker::PhantomData;

pub trait Token {
    type Base;
    const TOKEN: Self::Base;

    fn span(&self) -> Span;
}

#[derive(Debug)]
pub struct ParseTokenError<T: Token> {
    pub span: Span,
    _mark: PhantomData<T>,
}

macro_rules! declare_short_tokens {
    ($($name: ident = $expr: expr);* $(;)*) => {
        $(
            declare_short_tokens!(@impl $name = $expr);
        )*
    };

    (@impl $name: ident = $expr: expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub span: Span,
        }

        #[allow(unused)]
        impl $name {
            pub(crate) fn from_span(span: Span) -> Self {
                Self {
                    span
                }
            }
        }

        impl Token for $name {
            type Base = char;
            const TOKEN: Self::Base = $expr;

            fn span(&self) -> Span {
                self.span.clone()
            }
        }

        impl Parse for $name {
            type Error = ParseTokenError<Self>;
            type Fatal = Never;

            async fn parse<S: crate::InputStream>(
                input: S,
            ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
                let start = input.metrics();
                match any_char().parse(input).await {
                    Done(Self::TOKEN, r) => {
                        let end = r.metrics();
                        Done(
                            Self {
                                span: Span::new(start, end),
                            },
                            r,
                        )
                    }
                    Done(_, r) => Fail(
                        ParseTokenError {
                            span: Span::points(start),
                            _mark: PhantomData,
                        },
                        r.into(),
                    ),
                    Fail(_, r) => Fail(
                        ParseTokenError {
                            span: Span::points(start),
                            _mark: PhantomData,
                        },
                        r,
                    ),
                    Fatal(e, _) => e.never(),
                }
            }
        }
    };
}

macro_rules! declare_tokens {
    ($($name: ident = $expr: expr);* $(;)*) => {
        $(
            declare_tokens!(@impl $name = $expr);
        )*
    };

    (@impl $name: ident = $expr: expr) => {
        #[derive(Debug)]
        pub struct $name {
            pub span: Span,
        }

        #[allow(unused)]
        impl $name {
            pub(crate) fn from_span(span: Span) -> Self {
                Self {
                    span
                }
            }
        }

        impl Token for $name {
            type Base = &'static str;
            const TOKEN: Self::Base = $expr;

            fn span(&self) -> Span {
                self.span.clone()
            }
        }

        impl Parse for $name {
            type Error = ParseTokenError<Self>;
            type Fatal = Never;

            async fn parse<S: crate::InputStream>(
                input: S,
            ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
                let start = input.metrics();
                let rest = match atom(Self::TOKEN).parse(input).await {
                    Done(_, r) => r,
                    Fail(_, r) => {
                        return Fail(
                            ParseTokenError {
                                span: Span::points(start),
                                _mark: PhantomData,
                            },
                            r,
                        );
                    }
                    Fatal(e, _) => e.never(),
                };
                let end = rest.metrics();
                let span = Span::new(start, end);
                let me = Self { span };

                Done(me, rest)
            }
        }
    };
}

declare_short_tokens![
    Semi     = ';';
    Plus     = '+';
    Minus    = '-';
    Asterisk = '*';
    Slash    = '/';
    LParen   = '(';
    RParen   = ')';
    LBrace   = '{';
    RBrace   = '}';
    Eq       = '=';
    Comma    = ',';
];

declare_tokens![
    Let = "let";
];
