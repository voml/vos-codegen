use super::*;
use crate::parser::vos::FieldStatementNode;

impl FieldStatementNode {
    pub fn as_field(&self) -> VosResult<FieldStatement> {
        let mut field = FieldStatement::default();
        field.field = self.key.as_identifier();
        field.typing = self.r#type.as_field_type()?;
        // field.range = v.key.as_identifier();
        Ok(field)
    }
}
