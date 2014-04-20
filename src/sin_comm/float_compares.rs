#![macro_escape]

use std::f64::*;

// note that these macros give undefinted behavior for NaNs!
// if one of the arguments is NaN, the second argument is always returned as
// comparisons on NaN always return false
// we should probably disallow NaN in sinuous signals through some cleaning process.
macro_rules! max(
	($v1:expr, $v2:expr) => (
		if $v1 > $v2 {$v1} else {$v2}
	)
)

macro_rules! min(
	($v1:expr, $v2:expr) => (
		if $v1 < $v2 {$v1} else {$v2}
	)
)

#[test]
fn test_min_max(){
	let a: f64 = 0.0;
	let b: f64 = 2.0;
	assert_eq!(max!(a,b),b);
	assert_eq!(min!(a,b),a);

	let c: f32 = 1.0;
	let d: f32 = 3.0;
	assert_eq!(max!(c,d),d);
	assert_eq!(min!(c,d),c);
}

#[test]
fn test_nan(){
	let a: f64 = 1.0;
	assert!( !(a > NAN) );
	assert!( !(a < NAN) );
	assert!( !(a == NAN) );
}