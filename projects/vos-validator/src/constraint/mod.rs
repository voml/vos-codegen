use num::{BigInt, Signed, Zero};
use std::str::FromStr;
use vos_error::{VosError, VosResult};

mod decimal_constraint;
mod dict_constraint;
mod integer_constraint;
mod list_constraint;
mod string_constraint;

#[derive(Debug, Clone, Default)]
pub struct StringConstraint {
    /// Minimum length of utf8 string
    pub min_bytes: Option<u32>,
    /// Maximum length of utf8 string
    pub max_bytes: Option<u32>,
    /// Minimum number of unicode characters
    pub min_length: Option<u32>,
    /// Maximum number of unicode characters
    pub max_length: Option<u32>,
}

/// ```vos
/// n: i32[=1]
/// n: i32[<1]
/// n: i32[1..=2]
/// n: i32[1 < n < 2]
/// ```
#[derive(Debug, Clone)]
pub struct IntegerConstraint {
    /// Minimum length of utf8 string
    pub min: Option<BigInt>,
    /// Maximum length of utf8 string
    pub max: Option<BigInt>,
    /// Minimum number of unicode characters
    pub min_length: Option<BigInt>,
    /// Maximum number of unicode characters
    pub max_length: Option<BigInt>,
    /// Check if number is multiple of `x`
    pub multiple_of: Option<BigInt>,
}

#[derive(Debug, Clone)]
pub struct DecimalConstraint {
    /// Minimum length of utf8 string
    pub min: Option<BigInt>,
    /// Minimum number of unicode characters
    pub min_inclusive: bool,
    /// Maximum length of utf8 string
    pub max: Option<BigInt>,
    /// Maximum number of unicode characters
    pub max_inclusive: bool,
}

#[derive(Debug, Clone)]
pub struct ListConstraint {
    /// Minimum length of utf8 string
    pub min_length: Option<u32>,
    /// Maximum length of utf8 string
    pub max_length: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct DictConstraint {
    /// Minimum length of utf8 string
    pub min_length: Option<u32>,
    /// Maximum length of utf8 string
    pub max_length: Option<u32>,
}
