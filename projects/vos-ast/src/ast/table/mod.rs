use super::*;

impl Default for TableKind {
    fn default() -> Self {
        Self::Table
    }
}

impl Default for GenericStatement {
    fn default() -> Self {
        Self::Nothing
    }
}

impl TableStatement {
    pub fn add_field(&mut self, field: FieldStatement) -> Result<(), FieldStatement> {
        match self.fields.insert(field.name.to_string(), field) {
            None => Ok(()),
            Some(s) => Err(s),
        }
    }
    pub fn add_constraint(&mut self, constraint: ConstraintStatement) {
        self.constraints.insert(constraint.name.to_string(), constraint);
    }
}

impl FieldStatement {}

impl GenericStatement {
    pub fn operator(order: Ordering, inclusive: bool) -> char {
        match order {
            Ordering::Less if inclusive => '⩽',
            Ordering::Less => '<',
            Ordering::Equal => '=',
            Ordering::Greater if inclusive => '⩾',
            Ordering::Greater => '>',
        }
    }
}

impl Namespace {
    pub fn new(name: String, range: Span) -> Self {
        Self { scope: vec![Identifier { id: name, span: range }] }
    }
    pub fn push_identifier(&mut self, name: String, range: Span) {
        self.scope.push(Identifier { id: name, span: range })
    }
}

impl Identifier {}
