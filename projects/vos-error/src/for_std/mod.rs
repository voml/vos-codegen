use std::io::Error;

use crate::{VosError, VosErrorKind};

impl From<Error> for VosError {
    fn from(e: Error) -> Self {
        Self {
            kind: Box::new(VosErrorKind::IOError(e))
        }
    }
}