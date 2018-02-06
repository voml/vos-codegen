use diagnostic::TextStorage;
use vos_ast::parse_file;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut store = TextStorage::default();
    let file1 = store.file("tests/basic.vos").unwrap();
    let ast = parse_file(include_str!("basic.vos"));
    ast.eprint(&store).unwrap()
}
