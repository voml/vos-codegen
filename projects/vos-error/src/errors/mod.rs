use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use diagnostic::{DiagnosticLevel, FileID};

use crate::{DuplicateDeclare, VosError, VosErrorKind};

pub mod report;

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

impl DuplicateDeclare {
    pub fn build(self, file: FileID) -> VosError {
        VosError { kind: Box::new(VosErrorKind::DuplicateFields(self)), level: DiagnosticLevel::Warning, file }
    }
}
