mod vos;

use vos_error::VosError;


// #[test]
// fn test() {
//     let vos = VosParser::parse(r#"
// class A A
//
//     "#).unwrap();
//     println!("{:#?}", vos)
// }


struct VosVisitor {
    errors: Vec<VosError>
}

impl VosVisitor {

}

