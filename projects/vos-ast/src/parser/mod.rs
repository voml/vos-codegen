use std::{
    cmp::Ordering,
    fmt::Display,
    fs::read_to_string,
    ops::Range,
    path::{Path, PathBuf},
    str::FromStr,
};

use bigdecimal::BigDecimal;
use diagnostic::TextStorage;
use peginator::PegParser;

use vos_error::{Validation, VosError, VosResult};

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
    file: Option<PathBuf>,
    text: String,
    errors: Vec<VosError>,
}

impl FromStr for VosAST {
    type Err = VosError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match parse_file(s) {
            Validation::Success { value, diagnostics: _ } => Ok(value),
            Validation::Failure { fatal, diagnostics: _ } => Err(fatal),
        }
    }
}

pub fn parse_text<S>(path: S) -> Validation<VosAST>
where
    P: AsRef<Path>,
    S: Display,
{
    let file = path.as_ref().to_path_buf();
    let mut parser = VosVisitor { ast: VosAST { statements: vec![] }, text: "".to_string(), file, errors: vec![] };
    match parser.do_parse() {
        Ok(_) => {}
        Err(e) => return Validation::Failure { fatal: e, diagnostics: vec![] },
    }
    Validation::Success { value: parser.ast, diagnostics: parser.errors }
}

pub fn parse_file<P>(path: P) -> Validation<VosAST>
where
    P: AsRef<Path>,
{
    match VosVisitor::parse_file(path.as_ref().to_path_buf()) {
        Ok(o) => o,
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
    pub fn parse_file(file: PathBuf) -> VosResult<String> {
        let mut parser = Self { ast: Default::default(), file: Some(file.clone()), text: "".to_string(), errors: vec![] };
        match parser.do_parse() {
            Ok(o) => Ok(o),
            Err(e) => Err(e.with_path(file)),
        }
    }
    pub fn parse_text(text: String) -> VosResult<String> {
        let mut parser = Self { ast: Default::default(), file: None, text, errors: vec![] };
        parser.do_parse()
    }

    fn do_parse(&mut self) -> VosResult<String> {
        let mut file_id = self.read_file()?;
        for statement in VosParser::parse(&self.text)?.statements {
            match self.visit_statement(statement) {
                Ok(_) => {}
                Err(e) => self.errors.push(e),
            }
        }
        return Ok(file_id);
    }
    fn read_file(&mut self) -> VosResult<String> {
        match &self.file {
            Some(s) => {
                let file_id = TextStorage::canonicalize(s)?;
                self.text = read_to_string(&self.file)?;
                Ok(file_id)
            }
            None => {
                let file_id = "<anonymous>";
                Ok(file_id.to_string())
            }
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
