use crate::{Driver, GMarker, GParamItemWithMarker, GParamName, GSeparator, GType, ParseTree};

#[test]
fn test_separator() {
    let driver = Driver::from("");
    let x = GSeparator::derive(driver).unwrap();
    dbg!(x);

    let driver = Driver::from(" ");
    let x = GSeparator::derive(driver).unwrap();
    dbg!(x);

    let driver = Driver::from("  ");
    let x = GSeparator::derive(driver).unwrap();
    dbg!(x);

    let driver = Driver::from(" \n  \t");
    let x = GSeparator::derive(driver).unwrap();
    dbg!(x);
}

#[test]
fn test_types() {
    let driver = Driver::from("int");
    let der = GType::derive(driver).unwrap();
    assert_eq!(der.derived, GType::Int);

    let driver = Driver::from("bool ");
    let der = GType::derive(driver).unwrap();
    assert_eq!(der.derived, GType::Bool);

    let driver = Driver::from("char\n");
    let der = GType::derive(driver).unwrap();
    assert_eq!(der.derived, GType::Char);
}

#[test]
fn test_func_params_with_marker() {
    let driver = Driver::from("int _NOTNULL my_var");
    let der = GParamItemWithMarker::derive(driver).unwrap();
    assert_eq!(
        der.derived,
        GParamItemWithMarker {
            ty: GType::Int,
            marker: GMarker("_NOTNULL".to_string()),
            name: GParamName("my_var".to_string())
        }
    );

    let driver = Driver::from("bool\n_SOMEMARKER  some_bool");
    let der = GParamItemWithMarker::derive(driver).unwrap();
    assert_eq!(
        der.derived,
        GParamItemWithMarker {
            ty: GType::Bool,
            marker: GMarker("_SOMEMARKER".to_string()),
            name: GParamName("some_bool".to_string())
        }
    );
}