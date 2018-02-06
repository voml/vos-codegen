use indexmap::IndexMap;
use peginator::PegPosition;

use crate::parser::vos::{DictItem, ListItem, SpecialNode};

use super::*;

impl FieldStatementNode {
    pub fn as_field(&self) -> VosResult<FieldStatement> {
        let mut field = FieldStatement::default();
        field.name = self.key.as_identifier();
        field.typing = self.r#type.as_field_type()?;
        field.value = as_value(&self.value)?;
        Ok(field)
    }
}

impl ConstraintStatementNode {
    pub fn as_constraint(&self) -> VosResult<ConstraintStatement> {
        Ok(ConstraintStatement { name: self.key.as_identifier(), value: as_value(&self.value)? })
    }
}

impl ValueNode {
    pub fn as_value(&self) -> VosResult<ValueStatement> {
        match self {
            ValueNode::SpecialNode(v) => Ok(v.as_value()),
            ValueNode::NumNode(v) => v.as_value(),
            ValueNode::NamespaceNode(v) => Ok(v.as_value()),
            ValueNode::ListNode(v) => {
                let mut out = vec![];
                for item in &v.items {
                    match item {
                        ListItem::ValueNode(v) => out.push(v.as_value()?),
                        ListItem::Split(_) => {}
                    }
                }
                Ok(ValueStatement { kind: ValueKind::List(out), range: as_range(v.position()) })
            }
            ValueNode::DictNode(v) => {
                let mut out = IndexMap::default();
                for item in &v.items {
                    match item {
                        DictItem::KeyValueNode(v) => {
                            let key = v.key.as_identifier().id;
                            let value = v.value.as_value()?;
                            match out.insert(key, value) {
                                None => {}
                                Some(_) => {}
                            }
                        }
                        DictItem::Split(_) => {}
                    }
                }
                Ok(ValueStatement { kind: ValueKind::Dict(out), range: as_range(v.position()) })
            }
        }
    }
}

impl SpecialNode {
    pub fn as_value(&self) -> ValueStatement {
        let kind = match self.string.as_str() {
            "true" => ValueKind::Boolean(true),
            "false" => ValueKind::Boolean(false),
            _ => ValueKind::Null,
        };
        ValueStatement { kind, range: as_range(&self.position) }
    }
}

impl NumNode {
    pub fn as_num(&self) -> VosResult<BigDecimal> {
        Ok(BigDecimal::from_str(&self.string)?)
    }
    pub fn as_value(&self) -> VosResult<ValueStatement> {
        Ok(ValueStatement { kind: ValueKind::Number(self.as_num()?), range: as_range(&self.position) })
    }
}

impl NamespaceNode {
    pub fn as_namespace(&self) -> Namespace {
        let mut ns = Namespace::default();
        for id in &self.path {
            ns.push_identifier(id.as_string(), as_range(&id.position))
        }
        ns
    }
    pub fn as_value(&self) -> ValueStatement {
        ValueStatement { kind: ValueKind::Symbol(self.as_namespace()), range: as_range(&self.position) }
    }
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        let generic = GenericStatement::Arguments { arguments: vec![] };
        Ok(generic)
    }
}
