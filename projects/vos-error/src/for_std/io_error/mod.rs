use diagnostic::FileID;

use super::*;

impl From<Error> for VosError {
    fn from(error: Error) -> Self {
        Self { kind: Box::new(VosErrorKind::IOError(error)), level: DiagnosticLevel::Error, file: Default::default() }
    }
}

impl VosError {
    pub fn with_file(mut self, file: impl TryInto<FileID>) -> Self {
        match file.try_into() {
            Ok(o) => self.file = o,
            Err(_) => {}
        }
        self
    }
    pub fn set_file(&mut self, file: impl Into<FileID>) {
        self.file = file.into();
    }
}
