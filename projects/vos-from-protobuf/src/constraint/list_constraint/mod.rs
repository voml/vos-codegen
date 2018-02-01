use super::*;

impl Default for ListConstraint {
    fn default() -> Self {
        Self { min_length: None, max_length: None }
    }
}
