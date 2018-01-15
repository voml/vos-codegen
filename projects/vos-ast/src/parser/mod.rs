use peginator::PegParser;

use vos_error::{VosError, VosResult};

use crate::ast::{TableKind, TableStatement, VosAST, VosStatement};
use crate::parser::vos::{DeclareStatementNode, VosParser, VosStatementNode};

mod vos;

struct VosVisitor {
    ast: VosAST,
    errors: Vec<VosError>,
}

#[test]
fn test() {
    let vos = parse(r#"
class inline Color

    "#).unwrap();
    println!("{:#?}", vos)
}

pub fn parse(input: &str) -> Result<VosAST, Vec<VosError>> {
    let mut parser = VosVisitor {
        ast: VosAST { statements: vec![] },
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
            VosStatementNode::DeclareStatementNode(s) => {
                let stmt = VosStatement::Table(Box::new(s.visit(self)?));
                self.ast.statements.push(stmt)
            }
            VosStatementNode::DefineStatementNode(s) => {}
        }
        Ok(())
    }
}

impl DeclareStatementNode {
    fn declare(&self) -> &str {
        match self.modifiers.first() {
            None => { "" }
            Some(s) => {
                s.string.as_str()
            }
        }
    }
    fn visit(self, visitor: &mut VosVisitor) -> VosResult<TableStatement> {
        let kind = match self.declare() {
            "" => {
                return Err(VosError::parse_error(format!("declare keyword not found")));
            }
            "class" | "table" => { TableKind::Structure }
            "struct" | "structure" => { TableKind::Structure }
            _ => {
                return Err(VosError::parse_error(format!("`{}` not a valid declare keyword", self.declare())));
            }
        };

        Ok(TableStatement {
            kind
        })
    }
}

