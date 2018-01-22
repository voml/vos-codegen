use super::*;
use crate::parser::vos::GenericNum2Token;

impl GenericNum1 {
    pub fn as_int(&self) -> VosResult<BigInt> {
        let n = BigInt::from_str(&self.num.string)?;
        match self.token.as_order() {
            (Ordering::Equal, _) => {}
            (Ordering::Less, v) => {}
            (Ordering::Greater, v) => {}
        }
        Ok(n)
    }
}

impl GenericNum1Token {
    /// `(ordering, is_inclusive)`
    pub fn as_order(&self) -> (Ordering, bool) {
        match self.string.as_str() {
            "=" => (Ordering::Equal, true),
            ">" => (Ordering::Greater, false),
            ">=" | "⩾" | "≥" => (Ordering::Greater, true),
            "<" => (Ordering::Less, false),
            "<=" | "≤" | "⩽" => (Ordering::Less, true),
            _ => (Ordering::Equal, true),
        }
    }
}
impl GenericNum2Token {
    /// `(ordering, is_inclusive)`
    pub fn as_order(&self) -> (Ordering, bool) {
        match self.string.as_str() {
            "..=" => (Ordering::Equal, true),
            "..<" => (Ordering::Greater, false),
            ">=" | "⩾" | "≥" => (Ordering::Greater, true),
            "<" => (Ordering::Less, false),
            "<=" | "≤" | "⩽" => (Ordering::Less, true),
            _ => (Ordering::Equal, true),
        }
    }
}
