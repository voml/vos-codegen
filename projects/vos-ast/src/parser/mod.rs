use std::ops::Range;

use peginator::{PegParser, PegPosition};
use std::{cmp::Ordering, str::FromStr};

use num::BigInt;

use crate::{
    ast::{TableKind, TableStatement, VosAST, VosStatement},
    parser::vos::{
        DeclareBodyNode, GenericNode, GenericNum1, GenericNum1Token, IdentifierNode, KeyNode, NamespaceNode, TypeValueNode,
        VosParser, VosStatementNode,
    },
    FieldStatement, FieldTyping, GenericStatement, Namespace,
};
use vos_error::{VosError, VosResult};

mod number;
mod symbol;
mod vos;

struct VosVisitor {
    ast: VosAST,
    file: String,
    errors: Vec<VosError>,
}

#[test]
fn test() {
    let vos = parse(
        r#"
class Color {
    r: u8[<=9],
    g: u8[1..=2],
    b: u8[1<b<2],
}

table Color32 {
    r: f32,
}

    "#,
    )
    .unwrap();
    println!("{:#?}", vos)
}

pub fn parse(input: &str) -> Result<VosAST, Vec<VosError>> {
    let mut parser = VosVisitor { ast: VosAST { statements: vec![] }, file: "".to_string(), errors: vec![] };
    if let Err(e) = parser.parse(input) {
        return Err(vec![e]);
    }
    match parser.errors.is_empty() {
        true => Ok(parser.ast),
        false => Err(parser.errors),
    }
}

impl VosVisitor {
    pub fn parse(&mut self, input: &str) -> VosResult {
        for statement in VosParser::parse(input)?.statements {
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
        }
        Ok(())
    }
    fn push_table(&mut self, mut table: TableStatement, id: IdentifierNode, body: Vec<DeclareBodyNode>) -> VosResult {
        table.set_name(&id.string);
        for term in body {
            match term {
                DeclareBodyNode::FieldStatementNode(v) => {
                    // v.r#type.value
                    let ns = v.r#type.name.as_namespace();

                    FieldTyping { namespace: ns, generics: Default::default() };

                    match table.add_field(v.key.as_str(), v.key.position().clone()) {
                        Ok(_) => {}
                        Err(e) => {
                            todo!("重复的 key {}", e.field)
                        }
                    }
                }
                DeclareBodyNode::KeyValueDot(_) => {}
                DeclareBodyNode::Split(_) => {}
            }
        }
        self.ast.statements.push(VosStatement::Table(Box::new(table)));
        Ok(())
    }
}

impl DeclareBodyNode {
    fn visit(self, visitor: &mut VosVisitor) -> VosResult<FieldStatement> {
        todo!()
    }
}

impl KeyNode {
    pub fn as_str(&self) -> String {
        match self {
            KeyNode::IdentifierNode(v) => v.string.to_owned(),
            KeyNode::NumNode(v) => v.string.to_owned(),
        }
    }
}
