use diagnostic::DiagnosticLevel;
use std::{
    error::Error,
    fmt::{Display, Formatter},
};

pub mod report;

use crate::{VosError, VosErrorKind};

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
