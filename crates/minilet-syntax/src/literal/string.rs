use parcom::{
    ParcomSegmentIterator,
    ParseResult::{Done, Fail, Fatal},
};

use crate::{Parse, Span};

#[derive(Debug)]
pub struct StringLiteral {
    pub raw_text: String,
    pub text: String,
    pub span: Span,
}

impl Parse for StringLiteral {
    type Error = ParseStringLiteralError;
    type Fatal = ParseStringLiteralError;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> parcom::ParseResult<S, Self, Self::Error, Self::Fatal> {
        let mut state = StringParserState::Initial;

        let start = input.metrics();
        let mut segments = input.segments();

        let mut raw_text = String::new();
        let mut text = String::new();

        'outer: loop {
            let Some(segment) = segments.next(0).await else {
                match state {
                    StringParserState::Initial => {
                        return Fail(ParseStringLiteralError(Span::points(start)), input.into())
                    }
                    _ => return Fatal(ParseStringLiteralError(Span::points(start)), input.into()),
                }
            };

            for c in segment.chars() {
                match state {
                    StringParserState::Initial => match c {
                        '"' => {
                            state = StringParserState::Text;
                            raw_text.push(c);
                        }
                        _ => {
                            return Fail(ParseStringLiteralError(Span::points(start)), input.into())
                        }
                    },
                    StringParserState::Text => match c {
                        '\\' => {
                            state = StringParserState::Escape;
                            raw_text.push(c);
                        }
                        '"' => {
                            raw_text.push(c);
                            break 'outer;
                        }
                        _ => {
                            text.push(c);
                            raw_text.push(c);
                        }
                    },
                    StringParserState::Escape => match c {
                        '"' => {
                            state = StringParserState::Text;
                            raw_text.push(c);
                            text.push(c);
                        }
                        '\\' => {
                            state = StringParserState::Text;
                            raw_text.push(c);
                            text.push(c);
                        }
                        _ => {
                            return Fatal(
                                ParseStringLiteralError(Span::points(start)),
                                input.into(),
                            )
                        }
                    },
                }
            }
        }

        let rest = input.advance(raw_text.len()).await;
        let end = rest.metrics();

        let literal = StringLiteral {
            raw_text,
            text,
            span: Span::new(start, end),
        };

        Done(literal, rest)
    }
}

enum StringParserState {
    Initial,
    Text,
    Escape,
}

#[derive(Debug)]
pub struct ParseStringLiteralError(Span);

impl ParseStringLiteralError {
    pub fn span(&self) -> Span {
        self.0.clone()
    }
}
