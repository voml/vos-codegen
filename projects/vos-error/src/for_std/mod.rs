use std::{
    io::Error,
    num::{ParseFloatError, ParseIntError},
};

use crate::{VosError, VosErrorKind};

impl From<Error> for VosError {
    fn from(e: Error) -> Self {
        Self { kind: Box::new(VosErrorKind::IOError(e)), file: "".to_string(), range: None }
    }
}

impl From<ParseIntError> for VosError {
    fn from(e: ParseIntError) -> Self {
        Self { kind: Box::new(VosErrorKind::ParseError(e.to_string())), file: "".to_string(), range: None }
    }
}

impl From<ParseFloatError> for VosError {
    fn from(e: ParseFloatError) -> Self {
        Self { kind: Box::new(VosErrorKind::ParseError(e.to_string())), file: "".to_string(), range: None }
    }
}
