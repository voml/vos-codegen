use std::{
    error::Error,
    fmt::{Display, Formatter},
    ops::Range,
    path::PathBuf,
};

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
    pub fn file(&self) -> PathBuf {
        PathBuf::from(&self.file)
    }
    pub fn range(&self) -> Option<Range<u32>> {
        self.range.clone()
    }

    pub fn with_file(mut self, file_path: impl Into<String>) -> Self {
        self.set_file(file_path);
        self
    }
    pub fn set_file(&mut self, file_path: impl Into<String>) {
        self.file = file_path.into()
    }
    pub fn with_range(mut self, range: Range<u32>) -> Self {
        self.set_range(range);
        self
    }
    pub fn set_range(&mut self, range: Range<u32>) {
        self.range = Some(range)
    }
}

impl VosError {
    pub fn parse_error(message: impl Into<String>) -> Self {
        Self { kind: Box::new(VosErrorKind::ParseError(message.into())), file: "".to_string(), range: None }
    }
}
