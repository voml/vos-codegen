use vos_ast::parse_file;
use vos_error::{eprint, TextStorage};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let mut store = TextStorage::default();
    let file1 = store.file("tests/basic.vos").unwrap();
    print!("{}", file1);
    let ast = parse_file(include_str!("basic.vos"));
    eprint(&ast, &store).unwrap()
}
