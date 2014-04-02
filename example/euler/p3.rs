//! Problem 3 from project euler
//!
//! The prime factors of 13195 are 5, 7, 13 and 29.
//!
//! What is the largest prime factor of the number 600851475143 ?
///! Try a "brute force" "factor tree" method.  I think I remember this from
///! elementary school.

/// Define the structure of the tree - its a binary tree.  For a recursive
/// enum the contained versions must be pointers.  The tree head owns its
/// contents.
enum FactorTree {
	/// Case for a value and two branches
	Branching(uint, ~FactorTree, ~FactorTree),
	/// Case for a (nominally unfactored) leaf value.  This many not be necessary
	/// at all, and could possibly be removed by changing how we initialize the tree
	Leaf(uint),
	/// If the factor is a prime!
	Prime(uint)
}

/// find the smallest factorization of a number; this wrapper on the recursive
/// inner function is ugly but I'm not sure of a nicer way to do this
fn min_factorize(to_factor: uint) -> (uint, uint) {
	let start = 2;

	// recursively search for the smallest factor we can find
	fn min_factorize_recurse(to_factor: uint, start_at: uint) -> (uint, uint) {

		// if we have a factor
		if to_factor % start_at == 0 {
			(start_at, to_factor / start_at)
		}
		// if this is not a factor, incrememt the divisor
		else {
			min_factorize_recurse(to_factor, start_at+1)
		}
	}

	// handle edge case if we're being jerks
	if to_factor < start {
		(to_factor, to_factor)
	}
	// if not an edge case, factor it
	else {
		min_factorize_recurse(to_factor, start)
	}

}

/// run min_factorize and interpret the results as a FactorTree
fn return_factors_as_tree(to_factor: uint) -> ~FactorTree {

	// pattern match on the tuple of factors
	match min_factorize(to_factor) {
		// if the large factor is 1, to_factor is prime
		(to_factor, 1) => ~Prime(to_factor),
		// otherwise, return a tree with the value and its two factors
		(a, b) => ~Branching(to_factor, ~Leaf(a), ~Leaf(b))
	}
}

/// take a node and factor it if it is a leaf
fn factor_node(start_node: ~FactorTree) -> ~FactorTree {
	match start_node {
		~Leaf(val) => return_factors_as_tree(val),
		_ => start_node
	}
}

/// function to recursively build the factor tree
fn build_factor_tree(start_node: ~FactorTree) -> ~FactorTree {
	match start_node {
		// if the node is already a Prime, return the current tree
		~Prime(_) => start_node,
		// if the node is a leaf, factor it and work on the result
		~Leaf(_) => build_factor_tree(factor_node(start_node)),
		// if the node is a branch, recurse down each branch separately
		~Branching(val, left_branch, right_branch) => ~Branching(val, build_factor_tree(factor_node(left_branch)), build_factor_tree(factor_node(right_branch)))
	}
}

/// function to traverse the tree and collect a list of the primes
fn collect_primes(tree: ~FactorTree, mut prime_list: ~[uint]) -> ~[uint] {
	match tree {
		// if a prime, push it on the list
		~Prime(val) => {
			prime_list.push(val);
			prime_list },
		// we should not have leaves at this point; ignore them
		~Leaf(_) => prime_list,
		// if a branch, collect primes on both branches
		~Branching(_, left_branch, right_branch) => {
			prime_list = collect_primes(left_branch, prime_list);
			collect_primes(right_branch, prime_list)
		}
	}
}

fn main() {
	// seed the tree with a Leaf that we want to find the prime factors of
	let seed = ~Leaf(600851475143);

	// build the factor tree
	let the_tree = build_factor_tree(seed);

	// container for out primes
	let mut primes_list: ~[uint] = ~[];
	primes_list = collect_primes(the_tree, primes_list);

	// print the answer lazy-style
	println!("{:?}", primes_list);
}

#[test]
fn test_min_factorize_50(){

	fn check_factors(to_factor: uint, f1_check: uint, f2_check: uint) {
		let factors = min_factorize(to_factor);
		match factors {
			(f1, f2) => {
				assert_eq!(f1,f1_check);
				assert_eq!(f2,f2_check);
			}
		}
	}

	check_factors(50, 2, 25);
	check_factors(10, 2, 5);
	check_factors(1, 1, 1);
	check_factors(2,2,1);
	check_factors(3,3,1);
	check_factors(13,13,1);
}

/*
#[test]
fn test_return_factors_as_tree(){

	let a = 50;
	let result = match return_factors_as_tree(a) {
		~Prime(val) => (val, val),
		~Leaf(val) => (val, val),
		~Branching(val, )
	}

}
*/
