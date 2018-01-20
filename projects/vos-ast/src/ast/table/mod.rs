use super::*;

impl Default for TableStatement {
    fn default() -> Self {
        Self { kind: TableKind::Structure, name: "".to_string(), fields: Default::default(), constraints: Default::default() }
    }
}

impl Default for GenericStatement {
    fn default() -> Self {
        Self::Nothing
    }
}
impl Default for Namespace {
    fn default() -> Self {
        Self { scope: vec![] }
    }
}

impl TableStatement {
    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_string();
    }
    pub fn add_field(&mut self, key: String, range: Range<usize>) -> Result<(), FieldStatement> {
        let field = FieldStatement {
            field: key.clone(),
            typing: FieldTyping { namespace: Namespace { scope: vec![] }, generics: Default::default() },
            value: ValueStatement::Default,
            range,
        };
        match self.fields.insert(key, field) {
            None => Ok(()),
            Some(s) => Err(s),
        }
    }
    pub fn add_constraint(&mut self, key: String, range: Range<usize>) -> Result<(), FieldStatement> {
        let field = FieldStatement {
            field: key.clone(),
            typing: FieldTyping { namespace: Namespace { scope: vec![] }, generics: Default::default() },
            value: ValueStatement::Default,
            range,
        };
        match self.fields.insert(key, field) {
            None => Ok(()),
            Some(s) => Err(s),
        }
    }
}

impl Namespace {
    pub fn new(name: String, range: Range<u32>) -> Self {
        Self { scope: vec![(name, range)] }
    }
    pub fn push_id(&mut self, name: String, range: Range<u32>) {
        self.scope.push((name, range))
    }
}
