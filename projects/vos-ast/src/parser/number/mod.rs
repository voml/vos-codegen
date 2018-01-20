use std::{cmp::Ordering, str::FromStr};

use num::BigInt;

use crate::parser::vos::GenericNum1;

use super::*;

impl GenericNum1 {
    pub fn as_int(&self) -> VosResult<BigInt> {
        let n = BigInt::from_str(&self.num.string)?;
        match self.token.normalize() {
            (Ordering::Equal, _) => {}
            (Ordering::Less, v) => {}
            (Ordering::Greater, v) => {}
        }
        Ok(n)
    }
}

impl GenericNum1Token {
    /// `(ordering, is_inclusive)`
    pub fn normalize(&self) -> (Ordering, bool) {
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
