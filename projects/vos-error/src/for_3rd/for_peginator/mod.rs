use std::ops::Range;

use peginator::ParseError;

use crate::{VosError, VosErrorKind};

impl From<ParseError> for VosError {
    fn from(error: ParseError) -> Self {
        let p = error.position as u32;
        let e = VosErrorKind::ParseError(error.specifics.to_string());
        Self { kind: Box::new(e), file: "".to_string(), range: Some(Range { start: p, end: p }) }
    }
}
