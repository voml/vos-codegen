mod errors;
mod for_std;
mod for_3rd;

pub type VosResult<T = ()> = Result<T, VosError>;

#[derive(Debug)]
pub struct VosError {
    kind: Box<VosErrorKind>,
}

#[derive(Debug)]
pub enum VosErrorKind {
    IOError(std::io::Error),
    ParseError {
        message: String,
        file: String,
        range: Option<usize>,
    },
    UnknownError,
}