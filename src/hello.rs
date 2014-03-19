// The most predictable smoke test in the world.
// This version compiles with rustc 0.10-pre.
//
// To see how tests work (both successes and failures), compile and run with
// the --test flag:
//
//      $ rustc --test hello.rs
//      $ ./hello

fn main() {
    println!("Hello, world.");
}

#[test]
fn test_something() {
    println!("Testing...");
}

#[test]
fn test_fail() {
    println!("Testing failure...");
    fail!("Fail!");
}
