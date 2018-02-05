use std::{
    io::Error,
    num::{ParseFloatError, ParseIntError},
};

use diagnostic::DiagnosticLevel;

use crate::{IOError, VosError, VosErrorKind};

impl From<Error> for VosError {
    fn from(error: Error) -> Self {
        let e = IOError { error, source: Default::default() };
        Self { kind: Box::new(VosErrorKind::IOError(e)), level: DiagnosticLevel::Error }
    }
}

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
