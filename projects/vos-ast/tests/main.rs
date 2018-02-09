use std::path::PathBuf;
use vos_ast::parse;

use vos_error::{eprint, TextStorage};

#[test]
fn ready() {
    println!("it works!")
}
#[test]
fn here() {
    println!("{:?}", PathBuf::from("./").canonicalize());
}

#[test]
fn test_text() {
    let mut store = TextStorage::default();
    let file1 = store.anonymous(include_str!("basic.vos"));
    let text1 = store.get_text(&file1).unwrap();
    let ast1 = parse(text1, &file1);
    eprint(&ast1, &store).ok();
    println!("{:#?}", ast1.unwrap());
}

#[test]
fn test_file() {
    let mut store = TextStorage::default();
    let file1 = store.file("./tests/basic.vos").unwrap();
    let text1 = store.get_text(&file1).unwrap();
    let ast1 = parse(text1, &file1);
    eprint(&ast1, &store).ok();
    println!("{:#?}", ast1.unwrap());
}
