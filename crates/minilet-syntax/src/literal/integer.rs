use parcom::{
    Never, ParcomSegmentIterator,
    ParseResult::{Done, Fail},
};

use crate::{InputStream, Parse, Span};
#[derive(Debug)]
pub struct IntegerLiteral {
    pub prefix: Option<IntegerLiteralPrefix>,
    pub digits: String,
    pub number: i64,
    pub span: Span,
}

impl Parse for IntegerLiteral {
    type Error = ParseIntegerLiteralError;
    type Fatal = Never;

    async fn parse<S: InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let start = input.metrics();
        let mut segments = input.segments();
        let mut digits = String::new();
        let mut state = IntegerParserState::Initial;

        let mut prefix = None;

        'outer: loop {
            let Some(segment) = segments.next(0).await else {
                break;
            };

            for c in segment.chars() {
                match state {
                    IntegerParserState::Initial => match c {
                        '0' => {
                            state = IntegerParserState::MaybePrefixZero;
                        }
                        '0'..='9' => {
                            state = IntegerParserState::Digits;
                            digits.push(c);
                        }
                        _ => {
                            return Fail(
                                ParseIntegerLiteralError(Span::points(start)),
                                input.into(),
                            );
                        }
                    },
                    IntegerParserState::MaybePrefixZero => match c {
                        'b' => {
                            prefix = Some(IntegerLiteralPrefix::Bin);
                            state = IntegerParserState::Digits;
                        }
                        'o' => {
                            prefix = Some(IntegerLiteralPrefix::Oct);
                            state = IntegerParserState::Digits;
                        }
                        'x' => {
                            prefix = Some(IntegerLiteralPrefix::Hex);
                            state = IntegerParserState::Digits;
                        }
                        '0'..='9' => {
                            state = IntegerParserState::Digits;
                        }
                        _ => {
                            digits.push('0');
                            break 'outer;
                        }
                    },
                    IntegerParserState::Digits => match prefix {
                        Some(IntegerLiteralPrefix::Bin) if ('0'..='1').contains(&c) => {
                            digits.push(c);
                        }
                        Some(IntegerLiteralPrefix::Oct) if ('0'..='7').contains(&c) => {
                            digits.push(c);
                        }
                        Some(IntegerLiteralPrefix::Hex) if c.is_ascii_hexdigit() => {
                            digits.push(c);
                        }
                        None if c.is_ascii_digit() => {
                            digits.push(c);
                        }
                        _ => break 'outer,
                    },
                }
            }
        }

        let radix = match prefix {
            Some(IntegerLiteralPrefix::Bin) => 2,
            Some(IntegerLiteralPrefix::Oct) => 8,
            Some(IntegerLiteralPrefix::Hex) => 16,
            None => 10,
        };
        let Ok(num) = i64::from_str_radix(&digits, radix) else {
            return Fail(ParseIntegerLiteralError(Span::points(start)), input.into());
        };

        let consumed = match prefix {
            Some(_) => digits.len() + 2,
            None => digits.len(),
        };

        let rest = input.advance(consumed).await;
        let end = rest.metrics();
        let span = Span::new(start, end);

        let literal = IntegerLiteral {
            prefix,
            digits,
            number: num,
            span,
        };

        Done(literal, rest)
    }
}
#[derive(Debug)]
pub enum IntegerLiteralPrefix {
    Bin,
    Oct,
    Hex,
}

enum IntegerParserState {
    Initial,
    /// Starting `0` that may be prefix
    MaybePrefixZero,
    Digits,
}

#[derive(Debug)]
pub struct ParseIntegerLiteralError(Span);

impl ParseIntegerLiteralError {
    pub fn span(&self) -> Span {
        self.0.clone()
    }
}
