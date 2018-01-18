use peginator::PegParser;

use vos_error::{VosError, VosResult};

use crate::ast::{TableKind, TableStatement, VosAST, VosStatement};
use crate::FieldStatement;
use crate::parser::vos::{DeclareBodyNode, IdentifierNode, VosParser, VosStatementNode};

mod vos;

struct VosVisitor {
    ast: VosAST,
    file: String,
    errors: Vec<VosError>,
}

#[test]
fn test() {
    let vos = parse(r#"
class Color {
    r: u8
}

table Color32 {
    r: f32
}

    "#).unwrap();
    println!("{:#?}", vos)
}

pub fn parse(input: &str) -> Result<VosAST, Vec<VosError>> {
    let mut parser = VosVisitor {
        ast: VosAST { statements: vec![] },
        file: "".to_string(),
        errors: vec![],
    };
    if let Err(e) = parser.parse(input) {
        return Err(vec![e]);
    }
    match parser.errors.is_empty() {
        true => { Ok(parser.ast) }
        false => { Err(parser.errors) }
    }
}

impl VosVisitor {
    pub fn parse(&mut self, input: &str) -> VosResult {
        for statement in VosParser::parse(input)?.statements {
            match self.visit_statement(statement) {
                Ok(_) => {}
                Err(e) => { self.errors.push(e) }
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
        table.set_name(&id.string, &self.file, id.position);
        for term in body {
            match term {
                DeclareBodyNode::KeyValueDot(_) => {}
                DeclareBodyNode::KeyValueNode(_) => {}
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
