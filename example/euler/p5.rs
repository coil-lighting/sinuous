//! Problem 4 from project euler
//!
//! 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
//!
//! What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

// Only need to check multiples of 20
// Try next hardest number
// I don't care, brute force it

fn test_val_recursive(test_val: int, start_mult: int) -> bool {

	if start_mult == 1 {
		true
	}
	else if test_val % start_mult == 0 {
		test_val_recursive(test_val, start_mult - 1)
	}
	else {
		false
	}
}

fn main() {
	let mut i = 1;
	let mut test_val;
	let max_mult = 20;
	loop {
		test_val = i*max_mult;
		if test_val_recursive(test_val, max_mult) {
			break;
		}
		else {
			i += 1;
		}
	}

	println!("{}",test_val);

}

#[test]
fn test_test_val_recursive(){
	assert!(test_val_recursive(20,1));
	assert!(test_val_recursive(20,2));
	assert!(test_val_recursive(2520,10));
}