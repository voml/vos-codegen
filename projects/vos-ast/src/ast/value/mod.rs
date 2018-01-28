use super::*;

impl Default for ValueKind {
    fn default() -> Self {
        Self::Default
    }
}

impl PartialEq for ValueStatement {
    fn eq(&self, other: &Self) -> bool {
        self.kind.eq(&other.kind)
    }
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}
