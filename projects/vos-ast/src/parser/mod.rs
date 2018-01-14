use peginator::PegParser;

use vos_error::{VosError, VosResult};

use crate::ast::VosAST;
use crate::parser::vos::{VosParser, VosStatementNode};

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
            if let Err(e) = self.visit_statement(statement) { self.errors.push(e) }
        }
        return Ok(());
    }


    fn visit_statement(&mut self, node: VosStatementNode) -> VosResult {
        Ok(())
    }
}

