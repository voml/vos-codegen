use peginator::ParseError;

use crate::{VosError, VosErrorKind};

impl From<ParseError> for VosError {
    fn from(e: ParseError) -> Self {
        let error = VosErrorKind::ParseError {
            message: e.to_string(),
            file: "".to_string(),
            range: None,
        };
        Self {
            kind: Box::new(error)
        }
    }
}