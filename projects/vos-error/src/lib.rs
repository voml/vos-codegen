use std::ops::Range;

mod errors;
mod for_3rd;
mod for_std;

pub type VosResult<T = ()> = Result<T, VosError>;

#[derive(Debug)]
pub struct VosError {
    kind: Box<VosErrorKind>,
    file: String,
    range: Option<Range<u32>>,
}

#[derive(Debug)]
pub enum VosErrorKind {
    IOError(std::io::Error),
    ParseError(String),
    UnknownError,
}
