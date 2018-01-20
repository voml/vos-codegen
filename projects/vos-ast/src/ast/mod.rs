use std::{
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    ops::Range,
};

mod define;
mod display;
mod table;

#[derive(Debug, Clone)]
pub struct VosAST {
    pub statements: Vec<VosStatement>,
}

#[derive(Clone)]
pub enum VosStatement {
    Table(Box<TableStatement>),
}

#[derive(Debug, Clone)]
pub struct TableStatement {
    pub kind: TableKind,
    pub name: String,
    pub fields: BTreeMap<String, FieldStatement>,
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
pub struct FieldStatement {
    pub field: String,
    pub typing: ConstraintStatement,
    pub value: ValueStatement,
    pub range: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct ConstraintStatement {
    name: Namespace,
}

#[derive(Debug, Clone)]
pub struct Namespace {
    pub scope: Vec<String, Range<u32>>,
}

#[derive(Debug, Clone)]
pub enum ValueStatement {
    Default,
    String(String),
}
