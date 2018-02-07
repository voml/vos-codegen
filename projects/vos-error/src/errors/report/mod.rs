use diagnostic::{
    term::{
        emit,
        termcolor::{ColorChoice, StandardStream, WriteColor},
        TerminalConfig,
    },
    Diagnostic, DiagnosticLevel, FileID, TextStorage,
};

use crate::{DuplicateFields, Validation, VosError, VosErrorKind, VosResult};

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
            VosErrorKind::IOError(e) => {
                Diagnostic::new(self.level).with_code("A0002").with_message(e).with_primary(&self.file, 0..0, "IOError")
            }
            VosErrorKind::ParseError(e) => {
                Diagnostic::new(self.level).with_code("A0002").with_message(e).with_primary(&self.file, 0..0, "ParseError")
            }
            VosErrorKind::RuntimeError(e) => {
                Diagnostic::new(self.level).with_code("A0002").with_message(e).with_primary(&self.file, 0..0, "RuntimeError")
            }
            VosErrorKind::DuplicateFields(e) => e.as_report(self.level, &self.file),
            VosErrorKind::UnknownError => Diagnostic::new(self.level)
                .with_code("A0002")
                .with_message("UnknownError")
                .with_primary(&self.file, 0..0, "DuplicateFields"),
        }
    }
}

impl DuplicateFields {
    pub fn as_report(&self, level: DiagnosticLevel, file: &FileID) -> Diagnostic {
        Diagnostic::new(level) //
            .with_code("E0001")
            .with_message(&self.lhs.start)
            .with_primary(file, self.lhs.clone(), &self.lhs.end)
            .with_secondary(file, self.rhs.clone(), &self.lhs.end)
    }
}
