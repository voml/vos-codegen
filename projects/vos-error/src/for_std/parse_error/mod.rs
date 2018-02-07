use std::fmt::Display;

use super::*;

impl From<ParseIntError> for VosError {
    fn from(error: ParseIntError) -> Self {
        Self::parse_error(error.to_string())
    }
}

impl From<ParseFloatError> for VosError {
    fn from(error: ParseFloatError) -> Self {
        Self::parse_error(error.to_string())
    }
}

impl VosError {
    pub fn parse_error<S>(msg: S) -> Self
    where
        S: Display,
    {
        Self {
            kind: Box::new(VosErrorKind::ParseError(msg.to_string())),
            level: DiagnosticLevel::Error,
            file: Default::default(),
        }
    }
}
