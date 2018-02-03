use std::str::FromStr;

use vos_error::VosResult;

use super::*;

impl StringConstraint {
    /// ```vos
    /// type Integer: i32 {
    ///     .min: -1
    /// }
    /// ```
    pub fn min_bytes(&mut self, n: &str) -> VosResult {
        self.min_bytes = Some(u32::from_str(n)?);
        Ok(())
    }
    /// ```vos
    /// type Integer: i32 {
    ///     .max: +1
    /// }
    /// ```
    pub fn max_bytes(&mut self, n: &str) -> VosResult {
        self.max_bytes = Some(u32::from_str(n)?);
        Ok(())
    }
    /// ```vos
    /// type Positive: i32 {
    ///     .positive
    /// }
    /// ```
    /// ```vos
    /// type Integer: i32 {
    ///     .min: -1
    /// }
    /// ```
    pub fn min_length(&mut self, n: &str) -> VosResult {
        self.min_length = Some(u32::from_str(n)?);
        Ok(())
    }
    /// ```vos
    /// type Integer: i32 {
    ///     .max: +1
    /// }
    /// ```
    pub fn max_length(&mut self, n: &str) -> VosResult {
        self.max_length = Some(u32::from_str(n)?);
        Ok(())
    }
    /// ```vos
    /// type Positive: i32 {
    ///     .positive
    /// }
    /// ```
    pub fn format(&mut self) {}
}
