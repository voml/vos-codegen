use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use crate::{VosError, VosErrorKind};

pub mod report;

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
