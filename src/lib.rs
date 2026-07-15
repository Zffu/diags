//! The diags library is a library for diagnostics (errors, warnings, informations) and more!

use std::process::exit;

use crate::span::LabelledSpan;

pub mod fmt;
pub mod pos;
pub mod span;

#[derive(PartialEq)]
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

impl DiagnosticCode {
    pub fn new(level: Level, code: usize) -> Self {
        Self { level, code }
    }
}

impl Diagnostic {
    pub fn new(code: DiagnosticCode, message: String) -> Self {
        let diag = Self {
            code,
            message,
            primary_span: None,
            spans: vec![],
            notes: vec![],
            helps: vec![],
        };

        #[cfg(feature = "emit_on_creation")]
        {
            diag.emit();
        }

        diag
    }

    pub fn emit(&self) {
        println!("{}", self);

        if self.code.level == Level::CriticalError {
            exit(0); // Exists if the error is too much
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

pub trait IntoDiagnostic {
    fn into(self) -> Diagnostic;
}
