//! Demonstrate syntax for passing function "pointers" (really just functions).
//! This DOESN'T YET COMPILE, so obviously we don't quite get how to do this
//! with primitive traits.

fn apply_1(f: fn(i64) -> i64, n: i64) -> i64 {
    f(n)
}

fn square(n: i64) -> i64 {
    n * n
}

#[test]
fn test_apply_1() {
    assert!(apply_1(square, 2) == 4);
}

fn apply_2<T>(f: fn(T) -> T, n: T) -> T {
    f(n)
}

#[test]
fn test_apply_2() {
    assert!(apply_2(square, 2) == 4);
}


// ensure a is greater than b
fn sort_double<T>(a: Ord<T>, b: Ord<T>) -> (T, T) {
    if a < b {
        (b,a)
    } else {
        (a, b)
    }
}

// ensure a is greater than b
fn sort_apply<T>(f: fn(Ord<T>) -> T, a: Ord<T>, b: Ord<T>) -> (T, T) {
    sort_double(f(a), f(b))
}

#[test]
fn test_sort_double() {
    let (x, y) = sort_double(3,4);
    assert!(x == 4);
    assert!(y == 3);
}

#[test]
fn test_sort_apply() {
    let (x, y) = sort_apply(square, 3,4);
    assert!(x == 16);
    assert!(y == 9);
}
