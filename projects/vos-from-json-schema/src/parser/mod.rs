use serde_json::{Map, Value};
use vos_error::VosResult;

#[test]
fn test() {
    let root: Value = serde_json::from_str(include_str!("object.json")).unwrap();
    let mut reader = JsonSchemaReader {};
    reader.visit_root(root).unwrap();
}

pub struct JsonSchemaReader {}

impl JsonSchemaReader {
    fn visit_root(&mut self, root: Value) -> VosResult {
        match root {
            Value::Null => {
                todo!()
            }
            Value::Bool(_) => {
                todo!()
            }
            Value::Number(_) => {
                todo!()
            }
            Value::String(_) => {
                todo!()
            }
            Value::Array(_) => {
                todo!()
            }
            Value::Object(o) => self.visit_root_object(o),
        }
    }
    fn visit_root_object(&mut self, root: Map<String, Value>) -> VosResult {
        match root.get("type") {
            Some(Value::String(s)) => {}
            _ => {}
        }
        Ok(())
    }
}

use std::fs::File;
use valico::json_schema;

#[test]
fn main() {
    let json_v4_schema: Value = serde_json::from_str(include_str!("object.json")).unwrap();

    let mut scope = json_schema::Scope::new();
    let schema = scope.compile_and_return(json_v4_schema.clone(), false).unwrap();

    println!("{:#?}", schema.id);
    println!("{:#?}", schema.get_default());
    println!("Is valid: {:#?}", schema);
}
