use std::path::PathBuf;

use crate::pos::PositionDelimiter;

pub enum SpanKind {
    Primary,
    Seconday,
}

pub struct Span {
    pub kind: SpanKind,

    pub file: PathBuf,

    pub start_line: usize,
    pub start_col: usize,

    pub end_line: usize,
    pub end_col: usize,
}

pub struct LabelledSpan {
    pub span: Span,
    pub label: Option<String>,
}

impl Span {
    pub fn new(
        kind: SpanKind,
        file: PathBuf,
        start: PositionDelimiter,
        end: PositionDelimiter,
    ) -> Self {
        Self {
            kind,
            file,
            start_line: start.line,
            start_col: start.col,
            end_line: end.line,
            end_col: end.col,
        }
    }
}
