use std::fmt::Display;

use super::*;

impl Debug for VosStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VosStatement::Table(v) => Debug::fmt(v, f),
        }
    }
}

impl Debug for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let scope: Vec<_> = self.scope.iter().map(|(k, _)| k.as_str()).collect();
        f.write_str(&scope.join("."))
    }
}

impl Display for Namespace {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}
