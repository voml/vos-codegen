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
    NumberBound { symbol: Ordering, number: BigDecimal, inclusive: bool },
    NumberRange { min: BigDecimal, min_inclusive: bool, max: BigDecimal, max_inclusive: bool },
}

#[derive(Debug, Clone)]
pub struct ConstraintStatement {}

#[derive(Debug, Clone)]
pub struct ValueStatement {
    kind: ValueKind,
    range: Range<u32>,
}

#[derive(Clone)]
pub enum ValueKind {
    Default,
    String(String),
    Number(BigInt),
}

#[derive(Clone)]
pub struct Namespace {
    pub scope: Vec<Identifier>,
}

#[derive(Clone)]
pub struct Identifier {
    id: String,
    range: Range<u32>,
}
