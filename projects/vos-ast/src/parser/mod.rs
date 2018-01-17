use peginator::PegParser;

use vos_error::{VosError, VosResult};

use crate::ast::{Position, TableKind, TableStatement, VosAST, VosStatement};
use crate::parser::vos::{StructDeclareNode, TableDeclareNode, VosParser, VosStatementNode};

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
        file: "<anonymous>".to_string(),
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
                let stmt = s.visit(self)?;
                self.ast.statements.push(stmt)
            }
            VosStatementNode::TableDeclareNode(s) => {
                let stmt = s.visit(self)?;
                self.ast.statements.push(stmt)
            }
        }
        Ok(())
    }
}

impl StructDeclareNode {
    fn visit(self, visitor: &mut VosVisitor) -> VosResult<VosStatement> {
        let Self { id, body, .. } = self;
        Ok(VosStatement::Table(Box::new(TableStatement {
            kind: TableKind::Structure,
            name: id.string,
            name_position: Position::new(id.position, &visitor.file),
        })))
    }
}

impl TableDeclareNode {
    fn visit(self, visitor: &mut VosVisitor) -> VosResult<VosStatement> {
        let Self { id, body, .. } = self;
        Ok(VosStatement::Table(Box::new(TableStatement {
            kind: TableKind::Table,
            name: id.string,
            name_position: Position::new(id.position, &visitor.file),
        })))
    }
}

