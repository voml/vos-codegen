use std::error::Error;
use std::fmt::{Display, Formatter};

use crate::{VosError, VosErrorKind};

impl Display for VosError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl Error for VosError {}


impl VosError {
    pub fn kind(&self) -> &VosErrorKind {
        &*self.kind
    }
}