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

#[test]
fn sqrt_propagates_uncertainty() {
    let x = UncertainValue::new(4.0, 0.5);
    let y = x.sqrt();
    assert_eq!(y.nominal(), 2.0);
    // derivative 1/(2*sqrt(4)) = 1/4 -> variance = (0.5^2) * (1/4)^2 = 0.015625
    assert!((y.stddev() - 0.125).abs() < 1e-12);
}

#[test]
fn exp_propagates_uncertainty() {
    let x = UncertainValue::new(1.0, 0.1);
    let y = x.exp();
    assert!((y.nominal() - 2.718281828459045).abs() < 1e-12);
    assert!((y.stddev() - 0.27182818284590454).abs() < 1e-12);
}

#[test]
fn ln_propagates_uncertainty() {
    let x = UncertainValue::new(2.718281828459045, 0.1);
    let y = x.ln();
    assert!((y.nominal() - 1.0).abs() < 1e-12);
    assert!((y.stddev() - (0.1 / 2.718281828459045)).abs() < 1e-12);
}

#[test]
fn array_creation() {
    let arr = properr::uvals(vec![1.0, 2.0], vec![0.1, 0.2]);
    let noms = properr::nominals(arr.clone());
    let sigs = properr::stddevs(arr);
    assert_eq!(noms, vec![1.0, 2.0]);
    assert_eq!(sigs, vec![0.1, 0.2]);
}
