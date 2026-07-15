//! The diags library is a library for diagnostics (errors, warnings, informations) and more!

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
}
