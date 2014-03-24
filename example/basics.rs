use std::task::spawn;

fn constant_example() -> int {
    let x = 123;
    x
}

#[test]
fn test_constant_example() {
    assert!(constant_example() == 123)
}

/// Lend out an int to a function which will modify it as a side effect.
fn lend_param() -> int {
    let ~mut x = ~10; // start with an owned box (ptr) to an int
    vandalize_param(&mut x); // lend out x for the duration of the call. allow mutation.
    x
}

/// Show how to modify x in place as a side effect. Kinda like C syntax.
fn vandalize_param(x: &mut int) { // borrow x and expect to change it in-place.
    let y = *x * 2; // multiply the value of x
    *x = y; // set the value pointed to by x to equal the value of y
}

#[test]
fn test_lend_param() {
    assert!(lend_param() == 20)
}

fn lend_param2() -> int {
    let mut x = 15; // start with a plain local int
    vandalize_param2(&mut x); // lend out x for the duration of the call. allow mutation.
    x
}

#[test]
fn test_lend_param2() {
    assert!(lend_param2() == 30)
}

// Syntactical acrobatics. Don't do this.
fn vandalize_param2(x: &mut int) { // borrow x and expect to change it in-place.
    let mut y = *x * 2; // multiply the value of x

    // set the value pointed to by x to equal the value of a mutable borrowed
    // pointer to y
    *x = *&mut y;
}

// fn lend_param3() -> int {
//     let ~mut x = ~20; // start with an owned, mutable box containing 20
//     vandalize_param3(x); // lend out x for the duration of the call. allow mutation.
//     x
// }

// fn vandalize_param3(mut x: ~int) { // borrow x and expect to change it in-place.
//     let y = x * 2; // multiply the value of x
//     x = y; // set the value pointed to by x to equal the value of y
// }

// #[test]
// fn test_lend_param3() {
//     assert!(lend_param3() == 40)
// }

struct Point {x: f64, y: f64}

/// Demonstrate lifetime templatization of a function
fn lend_inner_variable<'r>(p: &'r Point) -> &'r f64 {
    &p.x // Return a pointer to a member of p sharing p's lifetime
}

#[test]
fn test_lend_inner_variable() {
    let p = Point {x: 12., y: 24.};
    let xx = lend_inner_variable(&p);
    assert!(*xx == 12.);
}

fn marco_polo() {
    print!("marco\n");
    print!("polo\n");
}

fn marco_polo_sequentially() {
    for _ in range(0,10) {
        marco_polo();
    }
}

// Load up all the CPU cores for a few seconds by marcopoloing a million times
fn marco_polo_concurrently() {
    for _ in range(0, 1000000) {
        // stdout always prints -- is it because tests only capture stdout from
        // their own tasks? ...and this is running (maybe partially?) in a
        // different set of tasks?
        spawn(marco_polo);
    }
}

#[test]
fn test_marco_polo_sequentially() {
    marco_polo_sequentially();
}

#[test]
fn test_marco_polo_concurrently() {
    marco_polo_concurrently();
}

fn main() {
    println!("Marco/polo should alternate:");
    marco_polo_sequentially();
    println!("\nMarco/polo may appear randomly due to concurrent execution:");
    marco_polo_concurrently();
}
