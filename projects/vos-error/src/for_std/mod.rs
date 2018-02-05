use std::{
    fs::read_to_string,
    io::Error,
    num::{ParseFloatError, ParseIntError},
};

use diagnostic::DiagnosticLevel;

use crate::{IOError, VosError, VosErrorKind, VosResult};
pub mod io_error;

impl From<ParseIntError> for VosError {
    fn from(error: ParseIntError) -> Self {
        let e = error.to_string();
        Self { kind: Box::new(VosErrorKind::ParseError(e)), level: DiagnosticLevel::Error }
    }
}

impl From<ParseFloatError> for VosError {
    fn from(error: ParseFloatError) -> Self {
        let e = error.to_string();
        Self { kind: Box::new(VosErrorKind::ParseError(e)), level: DiagnosticLevel::Error }
    }
}
