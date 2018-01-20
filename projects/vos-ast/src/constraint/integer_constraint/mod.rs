use super::*;

impl Default for IntegerConstraint {
    fn default() -> Self {
        Self { unsigned: false, min: None, max: None, min_length: None, max_length: None }
    }
}

impl IntegerConstraint {
    pub fn signed() -> IntegerConstraint {
        Self { unsigned: true, ..Default::default() }
    }
    pub fn unsigned() -> IntegerConstraint {
        Self { unsigned: false, ..Default::default() }
    }
}

impl IntegerConstraint {
    /// ```vos
    /// type Integer: i32 {
    ///     .min: -1
    /// }
    /// ```
    pub fn min(&mut self, n: &str, inclusive: bool) -> VosResult {
        let mut limit = BigInt::from_str(n)?;
        if !inclusive {
            limit += 1;
        }
        if self.unsigned && limit.is_negative() {
            self.min = Some(BigInt::zero());
            Err(VosError::parse_error(format!("{} less than zero", n)))
        }
        else {
            self.min = Some(limit);
            Ok(())
        }
    }
    /// ```vos
    /// type Integer: i32 {
    ///     .max: +1
    /// }
    /// ```
    pub fn max(mut self, n: &str, inclusive: bool) -> VosResult {
        let mut limit = BigInt::from_str(n)?;
        if !inclusive {
            limit -= 1;
        }
        if !self.unsigned && limit.is_negative() {
            self.max = Some(BigInt::zero());
            Err(VosError::parse_error(format!("{} less than zero", n)))
        }
        else {
            self.max = Some(limit);
            Ok(())
        }
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
        if self.unsigned { Err(VosError::parse_error("unsigned integer can't be negative")) } else { Ok(()) }
    }
}
