use crate::{VosError, VosErrorKind};
use num::bigint::ParseBigIntError;

impl From<ParseBigIntError> for VosError {
    fn from(e: ParseBigIntError) -> Self {
        Self { kind: Box::new(VosErrorKind::ParseError(e.to_string())), file: "".to_string(), range: None }
    }
}
