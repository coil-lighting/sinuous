// The most predictable smoke test in the world.
// This version compiles with rustc 0.10-pre.

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
