use super::*;

impl Debug for VosStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VosStatement::Table(v) => Debug::fmt(v, f),
        }
    }
}
