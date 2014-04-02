//! Problem 4 from project euler
//!
//! The sum of the squares of the first ten natural numbers is,
//!
//! 1^2 + 2^2 + ... + 10^2 = 385
//! The square of the sum of the first ten natural numbers is,
//!
//! (1 + 2 + ... + 10)^2 = 55^2 = 3025
//! Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
//!
//! Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

fn main() {

	let max = 100;
	let mut sum_squares = 0;
	let mut square_sum = 0;
	for i in range(1,max+1) {
		sum_squares += i*i;
		square_sum += i;
	}

	square_sum = square_sum*square_sum;

	println!("{}", square_sum - sum_squares);

}