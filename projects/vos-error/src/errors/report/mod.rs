use ariadne::{Color, Fmt, Label, Report, ReportKind, Source};
use std::{fs::read_to_string, path::PathBuf};

use crate::{DuplicateFields, IOError, Span, Validation, VosError, VosErrorKind, VosResult};

impl VosError {
    pub fn error(&mut self) {
        self.level = ReportKind::Error;
    }
    pub fn warning(&mut self) {
        self.level = ReportKind::Warning;
    }
    pub fn fatal(&mut self) {
        self.level = ReportKind::Custom("fatal", Color::Cyan);
    }
}

impl<T> Validation<T> {
    pub fn print(&self) {
        match self {
            Validation::Success { value: _, diagnostics } => {
                for diagnostic in diagnostics {
                    diagnostic.print()
                }
            }
            Validation::Failure { fatal, diagnostics } => {
                for diagnostic in diagnostics {
                    let file = diagnostic.file.as_str();
                    let src = Source::from(diagnostic.file);
                    diagnostic.as_report().eprint((file, src))
                }
                let file = fatal.file.as_str();
                let src = Source::from(fatal.file);
                fatal.as_report().eprint((file, src))
            }
        }
    }
}

impl VosError {
    pub fn print(&self) -> VosResult {
        let file = self.file.as_str();
        let report = self.as_report();
        match &self.file {
            None => report.eprint((file, src)),
            Some(s) => {
                let src = Source::from(read_to_string(s)?);
                report.eprint((file, src))
            }
        }
        Ok(())
    }

    pub fn as_report(&self) -> Report<(&str, Span)> {
        let file = self.file.as_str();
        let mut diag = Report::build(self.level, file, 0);
        match self.kind() {
            VosErrorKind::IOError(e) => {
                diag.set_message("IO Error");
                diag.set_note(e.to_string())
            }
            VosErrorKind::ParseError(e) => {
                diag.set_message("Parse Error");
                diag.set_note(e.to_string())
            }
            VosErrorKind::DuplicateFields(e) => {
                diag.set_message("Duplicate Fields Error");
                diag.add_label(e.lhs_label(file));
                diag.add_label(e.rhs_label(file));
                diag.set_note("Two fields with the same name cannot be declared duplicated")
            }
            VosErrorKind::UnknownError => {
                diag.set_message("UnknownError");
            }
        }
        diag.finish()
    }
}

impl IOError {
    pub fn as_report(&self, level: Lev) -> Report<(&str, Span)> {}
}

impl DuplicateFields {
    pub fn lhs_label(&self, file: &str) -> Label<(&str, Span)> {
        let c = Color::Fixed(80);
        Label::new((file, self.source.clone())).with_message(format!("{} first appeared here", self.symbol.fg(c))).with_color(c)
    }
    pub fn rhs_label(&self, file: &str) -> Label<(&str, Span)> {
        let c = Color::Fixed(81);
        Label::new((file, self.other.clone()))
            .with_message(format!("{} first duplicated here", self.symbol.fg(c)))
            .with_color(c)
    }
}
