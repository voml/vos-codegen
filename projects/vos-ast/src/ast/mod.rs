#[derive(Debug, Clone)]
pub struct VosAST {
    statements: Vec<VosStatement>,
}

#[derive(Debug, Clone)]
pub enum VosStatement {
    Table(Box<TableStatement>)
}

#[derive(Debug, Clone)]
pub enum TableKind {}

#[derive(Debug, Clone)]
pub struct TableStatement {
    kind: TableKind,
}
