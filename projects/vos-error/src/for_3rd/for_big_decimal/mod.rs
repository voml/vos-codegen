use crate::VosError;
use bigdecimal::ParseBigDecimalError;

impl From<ParseBigDecimalError> for VosError {
    fn from(error: ParseBigDecimalError) -> Self {
        // let p = error.position as u32;
        Self::parse_error(error.to_string())
    }
}
