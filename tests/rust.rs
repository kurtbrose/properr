use properr::UncertainValue;

#[test]
fn basic_arithmetic() {
    let x = UncertainValue::new(3.0, 1.0);
    let y = UncertainValue::new(1.0, 0.5);
    let z = &x - &y;
    assert_eq!(z.nominal(), 2.0);
}

