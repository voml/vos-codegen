use std::fmt::Display;

use diagnostic::{DiagnosticError, DiagnosticLevel};

use crate::{VosError, VosErrorKind};

#[cfg(feature = "bigdecimal")]
mod for_big_decimal;
#[cfg(feature = "num")]
mod for_num;
#[cfg(feature = "peginator")]
mod for_peg;

impl From<DiagnosticError> for VosError {
    fn from(error: DiagnosticError) -> Self {
        Self::runtime_error(error.to_string())
    }
}

impl VosError {
    pub fn runtime_error<S>(msg: S) -> Self
    where
        S: Display,
    {
        Self {
            kind: Box::new(VosErrorKind::RuntimeError(msg.to_string())),
            level: DiagnosticLevel::Info,
            file: Default::default(),
        }
    }
}
