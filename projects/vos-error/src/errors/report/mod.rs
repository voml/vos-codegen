use diagnostic::{
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream},
        TerminalConfig,
    },
    Diagnostic, DiagnosticLevel, FileID, TextStorage,
};

use crate::{DuplicateDeclare, Validation, VosError, VosErrorKind, VosResult};

impl VosError {
    pub fn with_level(mut self, level: DiagnosticLevel) -> Self {
        self.level = level;
        self
    }

    pub fn info(&mut self) {
        self.level = DiagnosticLevel::Info;
    }
    pub fn error(&mut self) {
        self.level = DiagnosticLevel::Error;
    }
    pub fn warning(&mut self) {
        self.level = DiagnosticLevel::Warning;
    }
    pub fn fatal(&mut self) {
        self.level = DiagnosticLevel::Fatal;
    }
}

pub fn eprint<T>(v: &Validation<T>, text: &TextStorage) -> VosResult {
    let c = TerminalConfig::default();
    let w: StandardStream = StandardStream::stderr(ColorChoice::Always);
    for diagnostic in v.collect_diagnostics() {
        emit(&mut w.lock(), &c, text, &diagnostic)?
    }
    Ok(())
}

impl From<&VosError> for Diagnostic {
    fn from(value: &VosError) -> Self {
        match value.kind() {
            VosErrorKind::IOError(e) => {
                Diagnostic::new(value.level).with_code("A0002").with_message(e).with_primary(&value.file, 0..0, "IOError")
            }
            VosErrorKind::ParseError(e) => {
                Diagnostic::new(value.level).with_code("A0002").with_message(e).with_primary(&value.file, 0..0, "ParseError")
            }
            VosErrorKind::RuntimeError(e) => {
                Diagnostic::new(value.level).with_code("A0002").with_message(e).with_primary(&value.file, 0..0, "RuntimeError")
            }
            VosErrorKind::DuplicateFields(e) => e.as_report(value.level, &value.file),
            VosErrorKind::UnknownError => Diagnostic::new(value.level)
                .with_code("A0002")
                .with_message("UnknownError")
                .with_primary(&value.file, 0..0, "DuplicateFields"),
        }
    }
}

impl DuplicateDeclare {
    pub fn as_report(&self, level: DiagnosticLevel, file: &FileID) -> Diagnostic {
        let message = format!("Duplicate {}", self.kind);
        let primary = format!("{} first declared here", self.kind);
        let secondary = format!("{} duplicate declared here again", self.kind);
        Diagnostic::new(level) //
            .with_message(message)
            .with_primary(file, self.lhs.clone(), primary)
            .with_secondary(file, self.rhs.clone(), secondary)
    }
}
