use parcom::metrics::LineColumn;

#[derive(Debug, Clone)]
pub struct Span {
    start: LineColumn,
    end: LineColumn,
}

impl Span {
    pub fn new(start: LineColumn, end: LineColumn) -> Self {
        Self { start, end }
    }

    pub fn points(point: LineColumn) -> Self {
        Self::new(point.clone(), point)
    }

    pub fn start(&self) -> &LineColumn {
        &self.start
    }

    pub fn end(&self) -> &LineColumn {
        &self.end
    }
}
