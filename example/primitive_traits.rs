//! Attempt to use primitive types in an enum. I find it hard to believe that
//! it required this much typing.

// TODO: use these in a trait.
// TODO: determine whether there is a simpler way to do express this.
// TODO: how does this relate to rust's Num, Primitive, etc?

fn a(n:f64) -> f64 {
    n*2.0
}

fn b(n:i64) -> i64 {
    n*3
}

fn c(n:u64) -> u64 {
    n*4
}

// The following derivation  auto-macro-generates == operators for the 3 types;
// without it you'll see this compiler error:
//     "error: binary operation `==` cannot be applied to type `Multipliable`".
// So 'deriving' basically generates something like this:
//     impl Eq for Foo {
//         fn eq(&self, other: &Foo) -> bool {
//             self.bar == other.bar
//         }
//     }
// ...but it must be different for primitive wrappers like IntMultipliable.
#[deriving(Eq)]
enum Multipliable {
    FloatMultipliable(f64),
    IntMultipliable(i64),
    UnsignedIntMultipliable(u64),
}

fn multiply(n: Multipliable) -> Multipliable {
    match n {
        FloatMultipliable(x) => FloatMultipliable(a(x)),
        IntMultipliable(x) => IntMultipliable(b(x)),
        UnsignedIntMultipliable(x) => UnsignedIntMultipliable(c(x))
    }
}

#[test]
fn test_abc() {
    let x: f64 = 123.4;
    let y: i64 = 234;
    let z: u64 = -345;

    assert!(a(x) == 246.8);
    assert!(b(y) == 702);
    assert!(c(z) == -1380);
}

#[test]
fn test_multiply() {
    let x: f64 = 123.4;
    let y: i64 = 234;
    let z: u64 = -345;

    // TODO how to just get those primitives back out of a Multipliable?
    // There's no way it can be this onerous.
    assert!(multiply(FloatMultipliable(x)) == FloatMultipliable(246.8));
    assert!(multiply(IntMultipliable(y)) == IntMultipliable(702));
    assert!(multiply(UnsignedIntMultipliable(z)) == UnsignedIntMultipliable(-1380));
}
