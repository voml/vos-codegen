use crate::{VosError, VosErrorKind};
use bigdecimal::ParseBigDecimalError;
use diagnostic::DiagnosticLevel;

impl From<ParseBigDecimalError> for VosError {
    fn from(error: ParseBigDecimalError) -> Self {
        // let p = error.position as u32;
        let e = error.to_string();
        Self { kind: Box::new(VosErrorKind::ParseError(e)), level: DiagnosticLevel::Error }
    }
}
