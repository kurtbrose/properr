use properr::UncertainValue;

#[test]
fn basic_arithmetic() {
    let x = UncertainValue::new(3.0, 1.0);
    let y = UncertainValue::new(1.0, 0.5);
    let z = &x - &y;
    assert_eq!(z.nominal(), 2.0);
}

#[test]
fn division_cancels_uncertainty() {
    let x = UncertainValue::new(3.0, 1.0);
    let y = &x / &x;
    assert_eq!(y.nominal(), 1.0);
    assert_eq!(y.stddev(), 0.0);
}

#[test]
fn sine_propagates_uncertainty() {
    let x = UncertainValue::new(0.0, 1.0);
    let y = x.sin();
    assert_eq!(y.nominal(), 0.0);
    assert_eq!(y.stddev(), 1.0);
}

#[test]
fn cosine_propagates_uncertainty() {
    let x = UncertainValue::new(0.0, 1.0);
    let y = x.cos();
    assert_eq!(y.nominal(), 1.0);
    assert_eq!(y.stddev(), 0.0);
}
