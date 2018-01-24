use crate::{VosError, VosErrorKind};
use bigdecimal::ParseBigDecimalError;

impl From<ParseBigDecimalError> for VosError {
    fn from(error: ParseBigDecimalError) -> Self {
        // let p = error.position as u32;
        let e = VosErrorKind::ParseError(error.to_string());
        Self { kind: Box::new(e), file: "".to_string(), range: None }
    }
}
