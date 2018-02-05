use diagnostic::{DiagnosticError, DiagnosticLevel};

use crate::{VosError, VosErrorKind};

#[cfg(feature = "bigdecimal")]
mod for_bigdecimal;
#[cfg(feature = "num")]
mod for_num;
#[cfg(feature = "peginator")]
mod for_peginator;

impl From<DiagnosticError> for VosError {
    fn from(e: DiagnosticError) -> Self {
        Self { kind: Box::new(VosErrorKind::RuntimeError(e.to_string())), level: DiagnosticLevel::Error }
    }
}
