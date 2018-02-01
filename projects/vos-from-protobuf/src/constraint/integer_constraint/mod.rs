use super::*;

impl Default for IntegerConstraint {
    fn default() -> Self {
        Self { min: None, max: None, min_length: None, max_length: None, multiple_of: None }
    }
}

impl IntegerConstraint {}

impl IntegerConstraint {
    /// ```vos
    /// i: i32[>1];
    /// i: i32[>=2];
    /// type Integer: i32 {
    ///     .min: -1
    /// }
    /// ```
    pub fn min(&mut self, n: &str, inclusive: bool) -> VosResult {
        let mut limit = BigInt::from_str(n)?;
        if !inclusive {
            limit += 1;
        }
        self.min = Some(limit);
        Ok(())
    }
    /// ```vos
    /// i: i32[<1];
    /// i: i32[<=0];
    /// type Integer: i32 {
    ///     .max: +1
    /// }
    /// ```
    pub fn max(mut self, n: &str, inclusive: bool) -> VosResult {
        let mut limit = BigInt::from_str(n)?;
        if !inclusive {
            limit -= 1;
        }
        self.max = Some(limit);
        Ok(())
    }
    /// ```vos
    /// type Positive: i32 {
    ///     .positive
    /// }
    /// ```
    pub fn positive(mut self) -> VosResult {
        self.min = Some(BigInt::zero());
        Ok(())
    }
    /// ```vos
    /// type Positive: i32 {
    ///     .positive
    /// }
    /// ```
    pub fn negative(mut self) -> VosResult {
        self.max = Some(BigInt::zero());
        Ok(())
    }
}
