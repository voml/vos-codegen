use std::fmt::{Debug, Formatter};
use std::ops::Range;

#[derive(Debug, Clone)]
pub struct VosAST {
    pub statements: Vec<VosStatement>,
}

#[derive(Clone)]
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
    pub name: String,
    pub name_position: Position,
}

#[derive(Clone)]
pub struct Position {
    pub start: u32,
    pub end: u32,
    pub file: String,
}

impl Debug for VosStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self { VosStatement::Table(v) => { Debug::fmt(v, f) } }
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.start, self.end)
    }
}

impl Position {
    pub fn new(range: Range<usize>, file: &str) -> Position {
        Position {
            start: range.start as u32,
            end: range.start as u32,
            file: file.to_string(),
        }
    }
}