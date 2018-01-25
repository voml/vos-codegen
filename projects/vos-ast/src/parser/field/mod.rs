use super::*;

impl FieldStatementNode {
    pub fn as_field(&self) -> VosResult<FieldStatement> {
        let mut field = FieldStatement::default();
        field.field = self.key.as_identifier();
        field.typing = self.r#type.as_field_type()?;
        match &self.value {
            Some(s) => field.value = s.as_value()?,
            None => {}
        }
        Ok(field)
    }
    fn as_value(&self) -> VosResult<ValueStatement> {
        match &self.value {
            Some(s) => s.as_value(),
            None => Ok(ValueStatement::default()),
        }
    }
}

impl ValueNode {
    pub fn as_value(&self) -> VosResult<ValueStatement> {
        match self {
            ValueNode::DefaultNode(v) => Ok(v.as_value()),
            ValueNode::BooleanNode(v) => Ok(v.as_value()),
            ValueNode::IdentifierNode(_) => {
                todo!()
            }
            ValueNode::NumNode(v) => {
                todo!()
            }
        }
    }
}

impl DefaultNode {
    pub fn as_value(&self) -> ValueStatement {
        ValueStatement { kind: ValueKind::Default, range: as_range(&self.position) }
    }
}

impl BooleanNode {
    pub fn as_bool(&self) -> bool {
        match self.string.as_str() {
            "true" => true,
            _ => false,
        }
    }
    pub fn as_value(&self) -> ValueStatement {
        ValueStatement { kind: ValueKind::Boolean(self.as_bool()), range: as_range(&self.position) }
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
