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
    pub fn with_file(mut self, file_path: impl Into<String>) -> Self {
        self.set_file(file_path);
        self
    }
    pub fn set_file(&mut self, file_path: impl Into<String>) {
        match self.kind.as_mut() {
            VosErrorKind::IOError(_) => {}
            VosErrorKind::ParseError { file, .. } => {
                *file = file_path.into()
            }
            VosErrorKind::UnknownError => {}
        }
    }
}

impl VosError {
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self {
            kind: Box::new(VosErrorKind::ParseError {
                message: message.into(),
                file: "".to_string(),
                range: None,
            })
        }
    }
}