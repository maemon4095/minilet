use crate::Parse;
use parcom::prelude::*;

#[derive(Debug)]
pub struct Punctured<T: Parse, P: Parse> {
    first: Option<T>,
    lasts: Vec<(P, T)>,
}

impl<T: Parse, P: Parse> Parse for Punctured<T, P> {
    type Error = Never;
    type Fatal = ParsePuncturedError<T, P>;

    async fn parse<S: crate::InputStream>(
        input: S,
    ) -> ParseResult<S, Self, Self::Error, Self::Fatal> {
        let mut lasts = Vec::new();

        let anchor = input.anchor();
        let (first, mut rest) = match T::parse(input).await {
            Done(v, r) => (v, r),
            Fail(_, r) => return Done(Self { first: None, lasts }, r.rewind(anchor)),
            Fatal(e, r) => return Fatal(ParsePuncturedError::Term(e), r),
        };

        loop {
            let anchor = rest.anchor();

            let punct = match P::parse(rest).await {
                Done(v, r) => {
                    rest = r;
                    v
                }
                Fail(_, r) => {
                    rest = r.rewind(anchor);
                    break;
                }
                Fatal(e, r) => return Fatal(ParsePuncturedError::Punct(e), r),
            };

            let term = match T::parse(rest).await {
                Done(v, r) => {
                    rest = r;
                    v
                }
                Fail(_, r) => {
                    rest = r.rewind(anchor);
                    break;
                }
                Fatal(e, r) => return Fatal(ParsePuncturedError::Term(e), r),
            };

            lasts.push((punct, term));
        }

        let me = Self {
            first: Some(first),
            lasts,
        };

        Done(me, rest)
    }
}

impl<T: Parse, P: Parse> Punctured<T, P> {
    pub fn first(&self) -> Option<&T> {
        self.first.as_ref()
    }

    pub fn last(&self) -> Option<&T> {
        self.lasts.last().map(|e| &e.1)
    }
}

#[derive(Debug)]
pub enum ParsePuncturedError<T: Parse, P: Parse> {
    Term(T::Fatal),
    Punct(P::Fatal),
}

#[derive(Debug)]
pub struct Iter<'a, T: Parse, P: Parse> {
    first: Option<&'a T>,
    lasts: <&'a [(P, T)] as IntoIterator>::IntoIter,
}

impl<'a, T: Parse, P: Parse> Iterator for Iter<'a, T, P> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.first
            .take()
            .or_else(|| self.lasts.next().map(|e| &e.1))
    }
}

#[derive(Debug)]
pub struct IntoIter<T: Parse, P: Parse> {
    first: Option<T>,
    lasts: <Vec<(P, T)> as IntoIterator>::IntoIter,
}

impl<T: Parse, P: Parse> Iterator for IntoIter<T, P> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.first.take().or_else(|| self.lasts.next().map(|e| e.1))
    }
}

impl<T: Parse, P: Parse> IntoIterator for Punctured<T, P> {
    type Item = T;
    type IntoIter = IntoIter<T, P>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            first: self.first,
            lasts: self.lasts.into_iter(),
        }
    }
}
