use crate::VosError;
use num::bigint::ParseBigIntError;

impl From<ParseBigIntError> for VosError {
    fn from(error: ParseBigIntError) -> Self {
        Self::parse_error(error.to_string())
    }
}
