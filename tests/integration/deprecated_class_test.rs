extern crate class_reader;

use crate::utils;
use utils::read_class_from_bytes;

#[test_log::test]
fn can_read_deprecated_attribute() {
    let class = read_class_from_bytes(include_bytes!("../resources/rjvm/DeprecatedClass.class"));
    assert!(class.deprecated);

    class.fields.get(0).unwrap();

    let field = class
        .fields
        .into_iter()
        .find(|f| f.name == "deprecatedField")
        .expect("should find field");
    assert!(field.deprecated);

    let method = class
        .methods
        .into_iter()
        .find(|m| m.name == "deprecatedMethod")
        .expect("should find method");
    assert!(method.deprecated);
}
