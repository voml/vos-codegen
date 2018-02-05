use super::*;
use std::path::PathBuf;

impl From<Error> for VosError {
    fn from(error: Error) -> Self {
        IOError { error: error.to_string(), source: Default::default() }
    }
}
impl From<IOError> for VosError {
    fn from(error: IOError) -> Self {
        Self { kind: Box::new(VosErrorKind::IOError(error)), level: DiagnosticLevel::Error }
    }
}

pub fn read_file_string(path: PathBuf) -> VosResult<String> {
    match read_to_string(&path) {
        Ok(o) => Ok(o),
        Err(e) => Err(IOError { error: e.to_string(), source: path }.into()),
    }
}
