//! problem 24 of project euler
//!
//! A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:
//!
//! 012   021   102   120   201   210
//!
//! What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

fn make_permutations(digits: &~[char]) -> ~[int] {
	let mut permutations_ints: ~[int] = ~[];
	let permutations: ~[~[char]] = digits.permutations().collect();
	for char_arr in permutations.iter() {
		permutations_ints.push( from_str(std::str::from_chars(*char_arr)).unwrap() );
	}
	permutations_ints
}

fn main(){

	let digits = ~['0', '1', '2','3','4','5','6','7','8','9'];

	let mut permutes = make_permutations(&digits);

	permutes.sort();

	println!("{}",permutes[1000000-1]);

}