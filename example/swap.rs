//! Explore rust's syntax for swapping values without temp variables, and
//! returning multiple values.
//!
//! Compare to `a, b = b, a` in Python.

fn swap_example_1(a: i64, b: i64) -> (i64, i64) {
    (b, a)
}

#[test]
fn test_swap_example_1() {
    let (x, y) = swap_example_1(1, 2);
    assert!(x == 2);
    assert!(y == 1);
}

fn swap_example_2<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

#[test]
fn test_swap_example_2_i64() {
    let a: i64 = 1;
    let b: i64 = 2;
    let (x, y) = swap_example_2(a, b);
    assert!(x == 2);
    assert!(y == 1);
}

#[test]
fn test_swap_example_2_f64() {
    let a: f64 = 1.0;
    let b: f64 = 2.0;
    let (x, y) = swap_example_2(a, b);
    assert!(x == 2.0);
    assert!(y == 1.0);
}

#[test]
fn test_swap_example_inline() {
    let a: bool = true;
    let b: bool = false;
    // ...but (a, b) = (b, a) doesn't work, even though it does in Python
    let (aa, bb) = (b, a);
    assert!(aa == false);
    assert!(bb == true);

    // In case they should be mutable, here's the syntax:
    let (mut aaa, mut bbb) = (b, a);
    aaa = true && aaa;
    bbb = false || bbb;
    assert!(aaa == false);
    assert!(bbb == true);
}
