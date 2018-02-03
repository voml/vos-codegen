use super::*;

impl VosAST {
    pub fn push_object(&mut self, name: Identifier, value: ValueStatement) {
        let o = ObjectStatement { name, value };
        self.statements.push(VosStatement::Object(Box::new(o)))
    }
}
