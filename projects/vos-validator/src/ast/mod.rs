use std::collections::BTreeMap;
use std::fmt::{Debug, Formatter};
use std::ops::Range;

mod display;
mod table;

#[derive(Debug, Clone)]
pub struct VosAST {
    pub statements: Vec<VosStatement>,
}

#[derive(Clone)]
pub struct Position {
    pub start: u32,
    pub end: u32,
    pub file: String,
}

#[derive(Clone)]
pub enum VosStatement {
    Table(Box<TableStatement>)
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
    pub value: ValueStatement,
    pub range: Range<usize>
}

#[derive(Debug, Clone)]
pub struct ConstraintStatement {
    pub field: String,
    pub value: ValueStatement,
    pub range: Range<usize>
}

#[derive(Debug, Clone)]
pub enum ValueStatement {
    Default,
    String(String),
}




impl Position {
    pub fn new(range: Range<usize>, file: &str) -> Position {
        Position {
            start: range.start as u32,
            end: range.end as u32,
            file: file.to_string(),
        }
    }
}