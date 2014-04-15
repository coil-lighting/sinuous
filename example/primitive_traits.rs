fn a(n:f64) -> f64 {
    n*2.0
}

fn b(n:i64) -> i64 {
    n*3
}

fn c(n:u64) -> u64 {
    n*4
}

enum Multipliable {
    FloatMultipliable(f64),
    IntMultipliable(i64),
    UnsignedIntMultipliable(u64),
}

fn multiply(n: Multipliable) {
    match n:
        FloatMultipliable(n) => a(n),
        IntMultipliable(n) => a(n),
        UnsignedIntMultipliable(n) => a(n),
}

#[test]
fn test_multiply() {
    let x = 123.4::f64
    let y = 234::i64
    let z = -345::u64

    assert!(multiply(x) == 246.8)
    assert!(multiply(y) == 702)
    assert!(multiply(z) == -1380)
}
