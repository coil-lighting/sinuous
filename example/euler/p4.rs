//! Problem 4 from project euler
//! A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
//!
//! Find the largest palindrome made from the product of two 3-digit numbers.

fn reverse_int(to_rev: int) -> int {
	let rev_string: ~str = to_rev.to_str().chars_rev().collect();
	let rev_int: int = from_str(rev_string).unwrap();
	rev_int
}

fn is_palindrome(number: int) -> bool {
	number == reverse_int(number)
}


fn main() {

	let mut largest = 0;

	for num1 in range(100,1000) {
		for num2 in range(100,1000) {
			let prod = num1 * num2;
			if is_palindrome(prod) && prod > largest {
				largest = prod;
			}
		}
	}

	println!("{}", largest);

}

#[test]
fn test_reverse_int(){
	assert_eq!(54321,reverse_int(12345));
	assert_eq!(1,reverse_int(1));
	assert_eq!(0,reverse_int(0));
	assert_eq!(1234321, reverse_int(1234321));
	assert_eq!(789, reverse_int(987));
}

#[test]
fn test_is_palindrome(){
	assert!(is_palindrome(1234321));
	assert!(!is_palindrome(100));
	assert!(is_palindrome(0));
	assert!(is_palindrome(1));
	assert!(is_palindrome(1000000000000000001));
}