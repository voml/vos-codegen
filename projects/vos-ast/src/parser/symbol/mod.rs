use bigdecimal::BigDecimal;

use super::*;

impl TypeValueNode {
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        match &self.generic {
            Some(s) => s.as_generic(),
            None => Ok(GenericStatement::Nothing),
        }
    }
}

impl GenericNode {
    pub fn as_generic(&self) -> VosResult<GenericStatement> {
        match self {
            GenericNode::GenericNum1(v) => {
                let number = BigDecimal::from_str(&v.num.string)?;
                let (symbol, inclusive) = v.token.as_order();
                let generic = GenericStatement::NumberBound { symbol, inclusive, number };
                Ok(generic)
            }
            GenericNode::GenericNum2(_) => {
                todo!()
            }
            GenericNode::GenericNum3(_) => {
                todo!()
            }
            GenericNode::IdentifierNode(_) => {
                todo!()
            }
        }
    }
}

impl NamespaceNode {
    pub fn as_namespace(&self) -> Namespace {
        let mut ns = Namespace::default();
        for id in &self.path {
            ns.push_id(id.as_symbol(), id.as_range())
        }
        ns
    }
}

impl IdentifierNode {
    pub fn as_symbol(&self) -> String {
        self.string.to_string()
    }
    pub fn as_range(&self) -> Range<u32> {
        Range { start: self.position.start as u32, end: self.position.end as u32 }
    }
}
