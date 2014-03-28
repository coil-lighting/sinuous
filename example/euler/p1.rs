//! Problem 1 from project euler
//! If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//!
//! Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
	let max = 999;

	let mut sum = 0;

	// find mults of 3 but not 5
	for val in range(1,1+1000/3) {
		let mult = val*3;
		if mult % 5 != 0 {
			sum += mult;
		}
	}

	for val in range(1,1000/5) {
		sum += val*5;
	}
	println!("{:?}",sum.to_str());
}