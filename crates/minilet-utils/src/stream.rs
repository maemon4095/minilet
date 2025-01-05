use parcom::{
    primitive::Nodes, IntoMeasured, MeasuredStream, Meter, Metrics, ParcomStream, RewindStream,
};

#[derive(Debug, Clone)]
pub struct StrStream<'me> {
    str: &'me str,
}

impl<'me> StrStream<'me> {
    pub fn new(str: &'me str) -> Self {
        Self { str }
    }
}

impl<'me> ParcomStream for StrStream<'me> {
    type Segment = str;
    type SegmentStream = Nodes<'me, str>;
    type Advance = std::future::Ready<Self>;

    fn segments(&self) -> Self::SegmentStream {
        self.str.segments()
    }

    fn advance(mut self, count: usize) -> Self::Advance {
        let mut chars = self.str.chars();
        for _ in 0..count {
            chars.next();
        }
        self.str = chars.as_str();
        std::future::ready(self)
    }
}
impl<'me> RewindStream for StrStream<'me> {
    type Anchor = Anchor<'me>;

    fn anchor(&self) -> Self::Anchor {
        Anchor {
            stream: self.clone(),
        }
    }

    fn rewind(self, anchor: Self::Anchor) -> Self {
        anchor.stream
    }
}

pub struct Anchor<'me> {
    stream: StrStream<'me>,
}

impl<'me, M> IntoMeasured<M> for StrStream<'me>
where
    M: Metrics<str>,
{
    type Measured = Measured<'me, M>;

    fn into_measured_with(self, meter: M::Meter) -> Self::Measured {
        Measured { meter, base: self }
    }
}

#[derive(Debug)]
pub struct Measured<'me, M: Metrics<str>> {
    meter: M::Meter,
    base: StrStream<'me>,
}

impl<'me, M> Clone for Measured<'me, M>
where
    M::Meter: Clone,
    M: Metrics<str>,
{
    fn clone(&self) -> Self {
        Self {
            meter: self.meter.clone(),
            base: self.base.clone(),
        }
    }
}

impl<'me, M: Metrics<str>> ParcomStream for Measured<'me, M> {
    type Segment = str;
    type SegmentStream = Nodes<'me, str>;
    type Advance = std::future::Ready<Self>;

    fn segments(&self) -> Self::SegmentStream {
        self.base.segments()
    }

    fn advance(mut self, count: usize) -> Self::Advance {
        let segment = self.base.str;
        let mut chars = segment.char_indices();
        chars.nth(count);
        let offset = chars.offset();
        self.meter = self.meter.advance(&segment[..offset]);
        self.base = self.base.advance(count).into_inner();
        std::future::ready(self)
    }
}

impl<'me, M> RewindStream for Measured<'me, M>
where
    M: Metrics<str>,
    M::Meter: Clone,
{
    type Anchor = MeasuredAnchor<'me, M>;

    fn anchor(&self) -> Self::Anchor {
        MeasuredAnchor {
            stream: self.clone(),
        }
    }

    fn rewind(self, anchor: Self::Anchor) -> Self {
        anchor.stream
    }
}

pub struct MeasuredAnchor<'me, M: Metrics<str>> {
    stream: Measured<'me, M>,
}

impl<'me, M: Metrics<str>> MeasuredStream for Measured<'me, M> {
    type Metrics = M;

    fn metrics(&self) -> Self::Metrics {
        self.meter.metrics()
    }
}
