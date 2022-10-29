use super::*;
impl Default for DecimalConstraint {
    fn default() -> Self {
        Self { min: None, min_inclusive: true, max: None, max_inclusive: true }
    }
}
