use vos_ast::{parse, VosAST};
use vos_error::VosError;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut text = match parse(include_str!("basic.vos")) {
        Ok(o) => {
            println!("{:#?}", o)
        }
        Err(e) => {}
    };
}
