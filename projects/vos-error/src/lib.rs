extern crate core;

use std::{ops::Range, path::PathBuf};

use diagnostic::DiagnosticLevel;

mod errors;
mod for_3rd;
mod for_std;

pub type Span = Range<usize>;

pub type VosResult<T = ()> = Result<T, VosError>;

#[derive(Debug)]
pub enum Validation<T> {
    Success { value: T, diagnostics: Vec<VosError> },
    Failure { fatal: VosError, diagnostics: Vec<VosError> },
}

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
    pub error: std::io::Error,
    pub source: PathBuf,
}

#[derive(Debug)]
pub struct DuplicateFields {
    pub path: String,
    pub symbol: String,
    pub lhs: Span,
    pub rhs: Span,
}
