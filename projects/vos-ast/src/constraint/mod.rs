use num::{BigInt, Zero};

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
/// type Positive: i16 {
///     .positive
///     .min: 1
///     .max: 2
/// }
/// ```
#[derive(Debug, Clone, Default)]
pub struct IntegerConstraint {
    /// Minimum length of utf8 string
    pub min: Option<BigInt>,
    /// Maximum length of utf8 string
    pub max: Option<BigInt>,
    /// Minimum number of unicode characters
    pub min_length: Option<BigInt>,
    /// Maximum number of unicode characters
    pub max_length: Option<BigInt>,
}

impl IntegerConstraint {
    /// ```vos
    /// type Integer: i32 {
    ///     .min: -1
    /// }
    /// ```
    pub fn min(mut self, n: BigInt) -> Self {
        self.min = Some(n);
        self
    }
    /// ```vos
    /// type Integer: i32 {
    ///     .max: +1
    /// }
    /// ```
    pub fn max(mut self, n: BigInt) -> Self {
        self.max = Some(n);
        self
    }
    /// ```vos
    /// type Positive: i32 {
    ///     .positive
    /// }
    /// ```
    pub fn positive(mut self) -> Self {
        self.min = Some(BigInt::zero());
        self
    }
}