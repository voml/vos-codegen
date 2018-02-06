use diagnostic::{
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream, WriteColor},
        TerminalConfig,
    },
    Diagnostic, DiagnosticLevel, FileID, TextStorage,
};

use crate::{DuplicateFields, IOError, Validation, VosError, VosErrorKind, VosResult};

impl VosError {
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
    match v {
        Validation::Success { value: _, diagnostics } => {
            for diagnostic in diagnostics {
                diagnostic.eprint(&mut w.lock(), &c, text)?
            }
        }
        Validation::Failure { fatal, diagnostics } => {
            for diagnostic in diagnostics {
                diagnostic.eprint(&mut w.lock(), &c, text)?
            }
            fatal.eprint(&mut w.lock(), &c, text)?
        }
    }
    Ok(())
}

impl VosError {
    pub fn eprint(&self, writer: &mut impl WriteColor, config: &TerminalConfig, text: &TextStorage) -> VosResult {
        emit(writer, config, text, &self.as_report())?;
        Ok(())
    }

    pub fn as_report(&self) -> Diagnostic {
        match self.kind() {
            VosErrorKind::IOError(e) => e.as_report(self.level),
            VosErrorKind::ParseError(e) => {
                Diagnostic::new(self.level).with_code("A0002").with_message(e).with_primary(anonymous(), 0..0, "")
            }
            VosErrorKind::RuntimeError(e) => {
                Diagnostic::new(self.level).with_code("A0002").with_message(e).with_primary(anonymous(), 0..0, "")
            }
            VosErrorKind::DuplicateFields(e) => e.as_report(self.level),
            VosErrorKind::UnknownError => {
                Diagnostic::new(self.level).with_code("A0002").with_message("UnknownError").with_primary(anonymous(), 0..0, "")
            }
        }
    }
}

impl IOError {
    pub fn as_report(&self, level: DiagnosticLevel) -> Diagnostic {
        let file_id = match FileID::from(&self.source) {
            Ok(o) => o,
            Err(_) => anonymous(),
        };
        Diagnostic::new(level).with_code("E0001").with_message(self.error.to_string()).with_primary(file_id, 0..0, "")
    }
}

impl DuplicateFields {
    pub fn as_report(&self, level: DiagnosticLevel) -> Diagnostic {
        let file_id = match FileID::from(&self.path) {
            Ok(o) => o,
            Err(_) => anonymous(),
        };
        Diagnostic::new(level).with_code("E0001").with_message(&self.lhs.start).with_primary(file_id, 0..0, &self.lhs.end)
    }
}

fn anonymous() -> String {
    "<anonymous>".to_string()
}
