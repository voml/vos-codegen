use std::ops::Range;

use ariadne::ReportKind;

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
    level: ReportKind,
}

#[derive(Debug)]
pub enum VosErrorKind {
    IOError(IOError),
    ParseError(String),
    DuplicateFields(DuplicateFields),
    UnknownError,
}

#[derive(Debug)]
pub struct IOError {
    error: std::io::Error,
    source: String,
}

#[derive(Debug)]
pub struct DuplicateFields {
    symbol: String,
    source: Span,
    other: Span,
}
