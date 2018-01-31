use std::{
    cmp::Ordering,
    collections::BTreeMap,
    fmt::{Debug, Formatter},
    ops::Range,
};

use bigdecimal::BigDecimal;
use indexmap::IndexMap;

mod constraint;
mod define;
mod display;
mod table;
mod value;

#[derive(Clone)]
pub struct VosAST {
    pub statements: Vec<VosStatement>,
}

#[derive(Clone)]
pub enum VosStatement {
    Table(Box<TableStatement>),
}

#[derive(Clone, Debug, Default)]
pub struct TableStatement {
    pub kind: TableKind,
    pub name: Identifier,
    pub fields: IndexMap<String, FieldStatement>,
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

#[derive(Clone, Default)]
pub struct FieldStatement {
    pub name: Identifier,
    pub typing: FieldTyping,
    pub value: ValueStatement,
}

#[derive(Clone, Default)]
pub struct FieldTyping {
    pub namespace: Namespace,
    pub generics: GenericStatement,
}

#[derive(Clone, PartialEq)]
pub enum GenericStatement {
    Nothing,
    NumberBound { symbol: Ordering, number: BigDecimal, inclusive: bool },
    NumberRange { min: BigDecimal, min_inclusive: bool, max: BigDecimal, max_inclusive: bool },
    Arguments { arguments: Vec<ValueStatement> },
}

#[derive(Debug, Clone)]
pub struct ConstraintStatement {
    pub name: Identifier,
    pub value: ValueStatement,
}

#[derive(Clone, Default)]
pub struct ValueStatement {
    pub kind: ValueKind,
    pub range: Range<u32>,
}

#[derive(Clone, PartialEq)]
pub enum ValueKind {
    Default,
    Boolean(bool),
    String(String),
    Number(BigDecimal),
    Symbol(Namespace),
    List(Vec<ValueStatement>),
    Dict(IndexMap<String, ValueStatement>),
}

#[derive(Clone, Default, PartialEq)]
pub struct Namespace {
    pub scope: Vec<Identifier>,
}

#[derive(Clone, Default)]
pub struct Identifier {
    pub id: String,
    pub range: Range<u32>,
}
