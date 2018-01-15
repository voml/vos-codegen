#[derive(Debug, Clone)]
pub struct VosAST {
    pub statements: Vec<VosStatement>,
}

#[derive(Debug, Clone)]
pub enum VosStatement {
    Table(Box<TableStatement>)
}

#[derive(Debug, Clone)]
pub enum TableKind {
    /// Compact structures, lower size and better performance, any changes will break compatibility
    ///
    /// - ❌ change order
    /// - ❌ add fields
    /// - ❌ change types
    /// - ❌ delete fields
    Structure,
    /// Structure with vtable, any changes will break compatibility
    ///
    /// - ✔️ change order
    /// - ✔️ add fields
    /// - ❌ change types
    /// - ❌ delete fields
    Table,
}

#[derive(Debug, Clone)]
pub struct TableStatement {
    pub kind: TableKind,
}
