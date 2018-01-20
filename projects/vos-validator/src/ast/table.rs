use super::*;

impl Default for TableStatement {
    fn default() -> Self {
        Self {
            kind: TableKind::Structure,
            name: "".to_string(),
            fields: Default::default(),
        }
    }
}

impl TableStatement {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn add_field(&mut self, key: String, range: Range<usize>) -> Result<(), FieldStatement> {
        let field = FieldStatement {
            field: key.clone(),
            value: ValueStatement::Default,
            range,
        };
        match self.fields.insert(key, field) {
            None => { Ok(()) }
            Some(s) => {
                Err(s)
            }
        }
    }
    pub fn add_constraint(&mut self, key: String, range: Range<usize>) -> Result<(), FieldStatement> {
        let field = FieldStatement {
            field: key.clone(),
            value: ValueStatement::Default,
            range,
        };
        match self.fields.insert(key, field) {
            None => { Ok(()) }
            Some(s) => {
                Err(s)
            }
        }
    }
}