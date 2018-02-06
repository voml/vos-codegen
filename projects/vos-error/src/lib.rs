use std::path::PathBuf;

use diagnostic::DiagnosticLevel;

pub use self::errors::report::eprint;

mod errors;
mod for_3rd;
mod for_std;

pub use diagnostic::Span;
pub type Validation<T> = diagnostic::Validation<T, VosError>;

pub type VosResult<T = ()> = Result<T, VosError>;

#[derive(Debug)]
pub struct VosError {
    kind: Box<VosErrorKind>,
    level: DiagnosticLevel,
}

#[derive(Debug)]
pub enum VosErrorKind {
    IOError(IOError),
    ParseError(String),
    RuntimeError(String),
    DuplicateFields(DuplicateFields),
    UnknownError,
}

#[derive(Debug)]
pub struct IOError {
    pub error: String,
    pub source: PathBuf,
}

#[derive(Debug)]
pub struct DuplicateFields {
    pub path: String,
    pub symbol: String,
    pub lhs: Span,
    pub rhs: Span,
}
