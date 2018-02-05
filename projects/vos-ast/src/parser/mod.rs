use std::{
    cmp::Ordering,
    fs::read_to_string,
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr,
};

use bigdecimal::BigDecimal;
use peginator::PegParser;

use vos_error::{IOError, Validation, VosError, VosResult};

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
    file: PathBuf,
    errors: Vec<VosError>,
}

impl FromStr for VosAST {
    type Err = VosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse(s) {
            Validation::Success { value, diagnostics: _ } => Ok(value),
            Validation::Failure { fatal, diagnostics: _ } => Err(fatal),
        }
    }
}

pub fn parse<P>(path: P) -> Validation<VosAST>
where
    P: AsRef<Path>,
{
    let file = path.as_ref().to_path_buf();
    let mut parser = VosVisitor { ast: VosAST { statements: vec![] }, file, errors: vec![] };
    match parser.parse(path) {
        Ok(_) => {}
        Err(e) => return Validation::Failure { fatal: e, diagnostics: vec![] },
    }
    Validation::Success { value: parser.ast, diagnostics: parser.errors }
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
    pub fn parse(&mut self) -> VosResult {
        let text = read_to_string(&self.file)?;

        for statement in VosParser::parse(input)?.statements {
            match self.visit_statement(statement) {
                Ok(_) => {}
                Err(e) => self.errors.push(e),
            }
        }
        return Ok(());
    }
    fn read_file(&mut self) -> VosResult<String> {
        match read_to_string(&self.file) {
            Ok(o) => Ok(o),
            Err(e) => IOError { error: e, source: self.file.clone() },
        }
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
                        todo!("重复的 key {}", e.name)
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
