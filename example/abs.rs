//! Explore rustc's absolute value function.

#[test]
fn test_abs() {
    let v: f64 = 0.0;
    assert!(v.abs() == 0.0);

    let w: i64 = 0;
    assert!(w.abs() == 0);

    let x: i64 = -1;
    assert!(x.abs() == 1);

    let y: i64 = 2;
    assert!(y.abs() == 2);

    let z: f64 = -5.5;
    assert!(z.abs() == 5.5);
}
