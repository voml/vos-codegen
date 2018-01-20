use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    ops::Range,
};

use bigdecimal::BigDecimal;
use num::BigInt;

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
    pub constraints: BTreeMap<String, ConstraintStatement>,
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
    pub typing: FieldTyping,
    pub value: ValueStatement,
    pub range: Range<usize>,
}

#[derive(Debug, Clone)]
pub struct FieldTyping {
    pub namespace: Namespace,
    pub generics: GenericStatement,
}

#[derive(Debug, Clone)]
pub enum GenericStatement {
    Nothing,
    NumberBound { symbol: Ordering, inclusive: bool, number: BigDecimal },
}

#[derive(Debug, Clone)]
pub struct ConstraintStatement {}

#[derive(Clone)]
pub struct Namespace {
    pub scope: Vec<(String, Range<u32>)>,
}

#[derive(Debug, Clone)]
pub enum ValueStatement {
    Default,
    String(String),
    Number(BigInt),
}
