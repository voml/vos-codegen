use std::str::FromStr;

use vos_ast::{VosAST, VosStatement};
use vos_error::{Validation, VosError, VosResult};

use crate::schema::Schema;

struct ParseState {
    error: Vec<VosError>,
    warn: Vec<VosError>,
}

impl Schema {
    pub fn parse(input: &str) -> Result<Validation<Self>, VosError> {
        let mut state = ParseState { error: vec![], warn: vec![] };
        match state.parse(input) {
            Ok(_) => {}
            Err(_) => {}
        }
        todo!()
    }
}

impl ParseState {
    fn parse(&mut self, i: &str) -> VosResult {
        let ast = VosAST::from_str(i)?;
        for statement in ast.statements {
            match statement {
                VosStatement::Table(_) => {}
                VosStatement::Object(_) => {}
                VosStatement::Union(_) => {}
            }
        }
        Ok(())
    }
}
