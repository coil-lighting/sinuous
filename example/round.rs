/// Explore rust's rounding behavior (much more to come!)

#[test]
fn test_round() {
    assert!(3.5f64.round() == 4.0);
    assert!(3.4f64.round() == 3.0);
}
