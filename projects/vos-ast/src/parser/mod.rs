use std::{cmp::Ordering, fmt::Display, ops::Range, str::FromStr};

use bigdecimal::BigDecimal;
use peginator::PegParser;

use vos_error::{DuplicateFields, FileID, Validation, VosError, VosResult};

use crate::{
    ast::{TableKind, TableStatement, VosAST, VosStatement},
    parser::vos::{
        ConstraintStatementNode, DeclareBodyNode, FieldStatementNode, GenericNode, GenericNum1, GenericNum1Token, GenericNum2,
        GenericNum2Token, GenericNum3, IdentifierNode, KeyNode, NamespaceNode, NumNode, TypeValueNode, ValueNode, VosParser,
        VosStatementNode,
    },
    ConstraintStatement, FieldStatement, FieldTyping, GenericStatement, Identifier, Namespace, ValueKind, ValueStatement,
};

mod field;
mod number;
mod symbol;
mod value;
mod vos;

struct VosVisitor {
    ast: VosAST,
    file_id: FileID,
    text: String,
    errors: Vec<VosError>,
}

pub fn parse<S>(text: S, id: &FileID) -> Validation<VosAST>
where
    S: Display,
{
    match VosVisitor::parse_text(text.to_string(), id.clone()) {
        Ok(o) => Validation::Success { value: o.ast, diagnostics: o.errors },
        Err(e) => Validation::Failure { fatal: e, diagnostics: vec![] },
    }
}

pub fn as_range(range: &Range<usize>) -> Range<u32> {
    Range { start: range.start as u32, end: range.end as u32 }
}

fn as_value(v: &Option<ValueNode>) -> VosResult<ValueStatement> {
    match v {
        Some(s) => s.as_value(),
        None => Ok(ValueStatement::default()),
    }
}

impl VosVisitor {
    pub fn parse_text(text: String, file_id: FileID) -> VosResult<Self> {
        let mut parser = Self { ast: Default::default(), file_id, text, errors: vec![] };
        parser.do_parse()?;
        Ok(parser)
    }

    fn do_parse(&mut self) -> VosResult {
        for statement in VosParser::parse(&self.text)?.statements {
            match self.visit_statement(statement) {
                Ok(_) => {}
                Err(e) => self.errors.push(e),
            }
        }
        return Ok(());
    }
    fn visit_statement(&mut self, node: VosStatementNode) -> VosResult {
        match node {
            VosStatementNode::StructDeclareNode(s) => {
                let mut table = TableStatement::default();
                table.kind = TableKind::Structure;
                self.push_table(table, s.id, s.body)?
            }
            VosStatementNode::TableDeclareNode(s) => {
                let mut table = TableStatement::default();
                table.kind = TableKind::Table;
                self.push_table(table, s.id, s.body)?
            }
            VosStatementNode::ObjectStatementNode(s) => self.ast.push_object(s.id.as_identifier(), s.value.as_value()?),
            VosStatementNode::UnionStatementNode(_) => {
                todo!()
                // s.id.as_identifier()
                // s.body
            }
            VosStatementNode::Split(_) => {}
        }
        Ok(())
    }
    fn push_table(&mut self, mut table: TableStatement, id: IdentifierNode, body: Vec<DeclareBodyNode>) -> VosResult {
        table.name = id.as_identifier();
        for term in body {
            match term {
                DeclareBodyNode::FieldStatementNode(v) => match table.add_field(v.as_field()?) {
                    Ok(_) => {}
                    Err(e) => {
                        let error =
                            DuplicateFields { symbol: e.name.id.to_string(), lhs: e.name.range, rhs: Default::default() };
                        self.errors.push(error.build(self.file_id.clone()))
                    }
                },
                DeclareBodyNode::ConstraintStatementNode(v) => table.add_constraint(v.as_constraint()?),
                DeclareBodyNode::Split(_) => {}
            }
        }
        self.ast.statements.push(VosStatement::Table(Box::new(table)));
        Ok(())
    }
}

impl KeyNode {
    pub fn as_identifier(&self) -> Identifier {
        match self {
            KeyNode::IdentifierNode(v) => v.as_identifier(),
            KeyNode::NumNode(_) => Identifier::default(),
        }
    }
}
