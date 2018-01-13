use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::VosError;

impl Display for VosError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for VosError {}


