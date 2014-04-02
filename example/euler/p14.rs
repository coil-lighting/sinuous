//! problem 14 from project euler
//!
//! The following iterative sequence is defined for the set of positive integers:
//!
//! n → n/2 (n is even)
//! n → 3n + 1 (n is odd)
//!
//! Using the rule above and starting with 13, we generate the following sequence:
//!
//! 13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
//! It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.
//!
//! Which starting number, under one million, produces the longest chain?
//!
//! NOTE: Once the chain starts the terms are allowed to go above one million.

fn sequence_size(number: int) -> int {

	match number {
		1 => 1,
		x if x % 2 == 0 => sequence_size(x/2) + 1,
		x => sequence_size(3*x + 1) + 1
	}

}

#[test]
fn test_sequence_size(){
	assert_eq!(sequence_size(13),10);
}

fn main() {
	let mut max_length = 0;
	let mut max_seed = 0;
	for start in range(1,1000000) {
		let length = sequence_size(start);
		if length > max_length {
			max_length = length;
			max_seed = start;
		}
	}

	println!("{}", max_seed);
}