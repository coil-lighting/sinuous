/// Demonstrate a compiler bug: u8 should not allow returning -1.

fn unsigned() -> u8 {
    -1
}

#[test]
fn test_unsigned() {
    let x:u8 = unsigned();
    assert!(x == -1);
}
