//! The diags library is a library for diagnostics (errors, warnings, informations) and more!

use crate::span::LabelledSpan;

pub mod pos;
pub mod span;

pub enum Level {
    CriticalError,
    Error,
    Warning,
    Info,
}

pub struct DiagnosticCode {
    pub level: Level,
    pub code: usize,
}

pub struct Diagnostic {
    pub code: DiagnosticCode,

    pub message: String,

    pub primary_span: Option<LabelledSpan>,
    pub spans: Vec<LabelledSpan>,

    pub notes: Vec<String>,
    pub helps: Vec<String>,
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, message: String) -> Self {
        Self {
            code,
            message,
            primary_span: None,
            spans: vec![],
            notes: vec![],
            helps: vec![],
        }
    }

    pub fn primary_span(self, primary_span: LabelledSpan) -> Self {
        let mut k = self;
        k.primary_span = Some(primary_span);

        k
    }

    pub fn span(self, span: LabelledSpan) -> Self {
        let mut k = self;
        k.spans.push(span);

        k
    }

    pub fn note(self, note: String) -> Self {
        let mut k = self;
        k.notes.push(note);

        k
    }

    pub fn help(self, help: String) -> Self {
        let mut k = self;
        k.helps.push(help);

        k
    }
}
