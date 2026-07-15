use std::path::PathBuf;

use crate::pos::PositionDelimiter;

pub struct MainSpan(pub LabelledSpan);
pub struct SecondarySpan(pub LabelledSpan);

pub enum SpanKind {
    Primary,
    Seconday,
}

#[derive(Clone, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct Span {
    pub file: PathBuf,

    pub start_line: usize,
    pub start_col: usize,

    pub end_line: usize,
    pub end_col: usize,
}

#[derive(Clone)]
#[cfg_attr(feature = "debug", derive(Debug))]
pub struct LabelledSpan {
    pub span: Span,
    pub label: Option<String>,
}

impl Span {
    pub fn new(file: PathBuf, start: PositionDelimiter, end: PositionDelimiter) -> Self {
        Self {
            file,
            start_line: start.line,
            start_col: start.col,
            end_line: end.line,
            end_col: end.col,
        }
    }

    pub fn new_start_only(file: PathBuf, position: PositionDelimiter) -> Self {
        Self {
            file,
            start_line: position.line,
            start_col: position.col,
            end_line: position.line,
            end_col: position.col,
        }
    }

    pub fn label(self, name: Option<String>) -> LabelledSpan {
        LabelledSpan {
            span: self,
            label: name,
        }
    }
}
