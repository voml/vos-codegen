
use super::*;

impl GenericNum1 {
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        let number = BigDecimal::from_str(&self.num.string)?;
        let (symbol, inclusive) = self.token.as_order();
        let generic = GenericStatement::NumberBound { symbol, inclusive, number };
        Ok(generic)
    }
}

impl GenericNum2 {
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        let min = BigDecimal::from_str(&self.num1.string)?;
        let max = BigDecimal::from_str(&self.num2.string)?;
        let min_order = self.token.as_min_order();
        let max_order = self.token.as_min_order();
        let generic = GenericStatement::NumberRange {
            min,
            min_symbol: min_order.0,
            min_inclusive: min_order.1,
            max,
            max_symbol: max_order.0,
            max_inclusive: max_order.1
        };
        Ok(generic)
    }
}

impl GenericNum3 {
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        let min = BigDecimal::from_str(&self.num1.string)?;
        let max = BigDecimal::from_str(&self.num2.string)?;
        let min_order = self.token1.as_order();
        let max_order = self.token2.as_order();
        let generic = GenericStatement::NumberRange {
            min,
            min_symbol: min_order.0,
            min_inclusive: min_order.1,
            max,
            max_symbol: max_order.0,
            max_inclusive: max_order.1
        };
        Ok(generic)
    }
}

impl GenericNum1Token {
    /// `(ordering, is_inclusive)`
    pub fn as_order(&self) -> (Ordering, bool) {
        match self.string.as_str() {
            ">" => (Ordering::Greater, false),
            ">=" | "⩾" | "≥" => (Ordering::Greater, true),
            "<" => (Ordering::Less, false),
            "<=" | "≤" | "⩽" => (Ordering::Less, true),
            "="| _ => (Ordering::Equal, true),
        }
    }
}
impl GenericNum2Token {
    /// `1..<2` => `1 < n <=2`
    pub fn as_min_order(&self) -> (Ordering, bool) {
        (Ordering::Less, true)
    }
    pub fn as_max_order(&self) -> (Ordering, bool) {
        match self.string.as_str() {
            "..<" => (Ordering::Less, false),
            "..="| _ => (Ordering::Less, true),
        }
    }
}
