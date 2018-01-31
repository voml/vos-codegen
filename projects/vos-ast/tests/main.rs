use vos_ast::parse;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let vos = parse(
        r#"
class Color {
    r: u8[<=9] = 8,
    g: u8[1..=2] = 7;
    b: u8[1<b<2] = 3,
    a: f32 = 1.0;
}
table Nest {
    color: Color = Red;
    ^require: [1, [2]]
    ^require: {a: 1}
}
    "#,
    )
    .unwrap();
    println!("{:#?}", vos)
}
