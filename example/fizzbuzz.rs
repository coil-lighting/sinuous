// A Fizzbuzz implementation, for Rust practice, and as a basic smoketest for
// the compiler, which we are compiling from head.
//
// This version compiles with rustc 0.10-pre.

fn divides_by_3(x: int) -> bool {
    x % 3 == 0
}

fn divides_by_5(x: int) -> bool {
    x % 5 == 0
}

#[test]
fn test_divides_by_3() {
    assert!(divides_by_3(3));
    assert!(divides_by_3(6));
    assert!(!divides_by_3(1));
    assert!(!divides_by_3(2));
    assert!(!divides_by_3(4));
}

#[test]
fn test_divides_by_5() {
    assert!(divides_by_5(5));
    assert!(divides_by_5(10));
    assert!(!divides_by_5(1));
    assert!(!divides_by_5(9));
    assert!(!divides_by_5(11));
}

#[test]
fn test_fizzbuzz_fmt() {
    assert!(fizzbuzz_fmt(1) == ~"1");
    assert!(fizzbuzz_fmt(2) == ~"2");
    assert!(fizzbuzz_fmt(3) == ~"Fizz");
    assert!(fizzbuzz_fmt(4) == ~"4");
    assert!(fizzbuzz_fmt(5) == ~"Buzz");
    assert!(fizzbuzz_fmt(6) == ~"Fizz");
    assert!(fizzbuzz_fmt(7) == ~"7");
    assert!(fizzbuzz_fmt(15) == ~"FizzBuzz");
}

fn fizzbuzz_fmt(x: int) -> ~str {
    if divides_by_3(x) {
        if divides_by_5(x) {
            ~"FizzBuzz"
        } else {
            ~"Fizz"
        }
    } else if divides_by_5(x) {
        ~"Buzz"
    } else {
        x.to_str()
    }
}

fn main() {
    println!("Fizzbuzz demo.");
    for x in range(1, 101) {
        println!("{}", fizzbuzz_fmt(x)); 
    }
}
