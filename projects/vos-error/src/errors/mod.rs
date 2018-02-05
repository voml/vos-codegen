use diagnostic::DiagnosticLevel;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

mod report;

use crate::{Validation, VosError, VosErrorKind};

impl Display for VosError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for VosError {}

impl VosError {
    pub fn kind(&self) -> &VosErrorKind {
        &*self.kind
    }
}

impl VosError {
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self { kind: Box::new(VosErrorKind::ParseError(message.into())), level: DiagnosticLevel::Error }
    }
}

impl<T> Validation<T> {
    pub fn no_problem(&self) -> bool {
        match self {
            Validation::Success { diagnostics, .. } => diagnostics.is_empty(),
            Validation::Failure { .. } => false,
        }
    }
    pub fn is_success(&self) -> bool {
        matches!(self, Validation::Success { .. })
    }
    pub fn is_failure(&self) -> bool {
        matches!(self, Validation::Failure { .. })
    }
}
