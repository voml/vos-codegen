pub use diagnostic::{DiagnosticLevel, FileID, Span, TextStorage};

pub use self::errors::report::eprint;

mod errors;
mod for_3rd;
mod for_std;

pub type Validation<T> = diagnostic::Validation<T, VosError>;

pub type VosResult<T = ()> = Result<T, VosError>;

#[derive(Debug)]
pub struct VosError {
    kind: Box<VosErrorKind>,
    level: DiagnosticLevel,
    file: FileID,
}

#[derive(Debug)]
pub enum VosErrorKind {
    IOError(std::io::Error),
    ParseError(String),
    RuntimeError(String),
    DuplicateFields(DuplicateDeclare),
    UnknownError,
}

#[derive(Debug)]
pub struct DuplicateDeclare {
    pub kind: &'static str,
    pub symbol: String,
    pub lhs: Span,
    pub rhs: Span,
}
