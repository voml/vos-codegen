use super::*;

impl TypeValueNode {
    pub fn as_field_type(&self) -> VosResult<FieldTyping> {
        Ok(FieldTyping { namespace: self.name.as_namespace(), generics: self.as_generic()? })
    }
    fn as_generic(&self) -> VosResult<GenericStatement> {
        match &self.generic {
            Some(s) => s.as_generic(),
            None => Ok(GenericStatement::Nothing),
        }
    }
}

impl GenericNode {
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        match self {
            GenericNode::GenericNum1(v) => v.as_generic(),
            GenericNode::GenericNum2(v) => v.as_generic(),
            GenericNode::GenericNum3(v) => v.as_generic(),
            GenericNode::NamespaceNode(v) => v.as_generic(),
        }
    }
}

impl IdentifierNode {
    pub fn as_string(&self) -> String {
        self.string.to_string()
    }
    pub fn as_identifier(&self) -> Identifier {
        Identifier { id: self.as_string(), range: as_range(&self.position) }
    }
}
