use std::path::PathBuf;

use super::*;

impl From<Error> for VosError {
    fn from(error: Error) -> Self {
        IOError { error: error.to_string(), source: Default::default() }.into()
    }
}

impl From<IOError> for VosError {
    fn from(error: IOError) -> Self {
        Self { kind: Box::new(VosErrorKind::IOError(error)), level: DiagnosticLevel::Error }
    }
}

impl VosError {
    pub fn with_path(mut self, path: PathBuf) -> VosError {
        self.set_path(path);
        self
    }
    pub fn set_path(&mut self, path: PathBuf) {
        match self.kind.as_mut() {
            VosErrorKind::IOError(i) => i.source = path,
            VosErrorKind::ParseError(_) => {}
            VosErrorKind::RuntimeError(_) => {}
            VosErrorKind::DuplicateFields(_) => {}
            VosErrorKind::UnknownError => {}
        }
    }
}
