use std::{
    io::Error,
    num::{ParseFloatError, ParseIntError},
};

use diagnostic::DiagnosticLevel;

use crate::{VosError, VosErrorKind};

mod io_error;
mod parse_error;
