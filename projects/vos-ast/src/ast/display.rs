use std::fmt::Display;

use super::*;

impl Debug for VosStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VosStatement::Table(v) => Debug::fmt(v, f),
        }
    }
}

impl Debug for ValueKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKind::Default => f.write_str("default"),
            ValueKind::Boolean(v) => write!(f, "{}", v),
            ValueKind::String(v) => write!(f, "{:?}", v),
            ValueKind::Number(v) => write!(f, "{}", v),
        }
    }
}

impl Debug for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let scope: Vec<_> = self.scope.iter().map(|v| v.id.as_str()).collect();
        f.write_str(&scope.join("."))
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Debug for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.id)
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
