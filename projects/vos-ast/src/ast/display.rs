use std::fmt::Display;

use super::*;

impl Debug for VosAST {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.statements.iter()).finish()
    }
}

impl Debug for VosStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            VosStatement::Table(v) => Debug::fmt(v, f),
        }
    }
}

impl Debug for GenericStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GenericStatement::Nothing => f.write_str("[]"),
            GenericStatement::NumberBound { symbol, number, inclusive } => {
                write!(f, "[{}{}]", GenericStatement::operator(*symbol, *inclusive), number,)
            }
            GenericStatement::NumberRange { min, min_inclusive, max, max_inclusive } => {
                write!(
                    f,
                    "[{} {} _ {} {}]",
                    min,
                    GenericStatement::operator(Ordering::Less, *min_inclusive),
                    GenericStatement::operator(Ordering::Less, *max_inclusive),
                    max
                )
            }
            GenericStatement::Arguments { arguments } => f.debug_list().entries(arguments.iter()).finish(),
        }
    }
}

impl Debug for FieldStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut w = &mut f.debug_struct("FieldStatement");
        w = w.field("type", &self.typing);
        w = w.field("value", &self.value);
        w.finish()
    }
}

impl Debug for FieldTyping {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.generics == GenericStatement::Nothing {
            true => Debug::fmt(&self.namespace, f),
            false => f.debug_struct("FieldType").field("namespace", &self.namespace).field("generics", &self.generics).finish(),
        }
    }
}

impl Debug for ValueStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.kind, f)
    }
}

impl Debug for ValueKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValueKind::Default => f.write_str("default"),
            ValueKind::Boolean(v) => write!(f, "{}", v),
            ValueKind::String(v) => write!(f, "{:?}", v),
            ValueKind::Number(v) => write!(f, "{}", v),
            ValueKind::Symbol(v) => write!(f, "{}", v),
            ValueKind::List(v) => f.debug_list().entries(v.iter()).finish(),
            ValueKind::Dict(v) => f.debug_map().entries(v.iter()).finish(),
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
