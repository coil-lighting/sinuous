use std::cmp::*;

// big TODO: determine if macros would make this much, much easier.

// type to contain all of the possible types we want to think about
// this may get large
// we may need to complicate the picture a bit
// just a few examples to start
#[deriving(Clone)]
pub enum Blendable {
	FlUni(f64), // float on the range 0 to 1
	FlBi(f64), // float on the range -1 to 1
	FlRange(f64,(f64,f64)), // float in a different range; this may be useful for things like strobe rate
	Set(i64,(i64,i64)), // set of possible integer values between min and max, defined by tuple (inclusive)
	ColorRGB(u8,u8,u8) // ok this kind of thing will be hard to blend into another, and really this should contain a more rich Color type for interchangeable representations
}

/*
macro_rules! coerce(
	($to_coerce:ident to $coerce_to:ident over [$($blend_type:ident),*]) => (
		match $coerce_to {
			$(
				$blend_type(_) => concat_idents!(to_,$blend_type)()
			)*
			_ => fail!()
		}
	);
)
*/

// some methods on blendable types
// if we have N blendable types, we have N^2 possible blend functions to implement.
// perhaps better: define blend functions for same types, and coerce functions for all to all
impl Blendable {
	// blend with another Blendable given a blend mode
	// output type is the same as the type of the caller
	// TODO: implement this function
	pub fn blend_over(&self, bw: Blendable, mode: BlendMode) -> Blendable {
		let bw_coerced = bw.coerce_to(*self);
		match mode {
			Clobber => *self,
			Max => blend_Max(*self, bw),
			Mean => blend_Mean(*self, bw),
			_ => fail!()
		}
	}


	// coerce a blendable to a unipolar float
	// this is a nice intermediate type to use in other to float coercions
	pub fn to_FlUni(&self) -> Blendable {
		match *self {
			FlUni(_) => *self,
			FlBi(v) => FlUni((v+1.)/2.),
			FlRange(v,(low,high)) => FlUni( (v - low)/(high - low) ),
			Set(v,(low,high)) => FlUni( ((v - low) as f64) / ( (high - low) as f64) ),
			ColorRGB(r,g,b) => FlUni ( (max( r, max(g,b) ) as f64) / 255. ) // for now use Value
		}
	}

	// coerce a blendable to a bipolar float
	pub fn to_FlBi(&self) -> Blendable {
		match *self {
			FlBi(_) => *self,
			to_coerce => FlBi( to_coerce.to_FlUni().unwrap_FlUni() *2. - 1. )
		}
	}

	// coerce a blendable to a float of arbitrary range, this one has to take a range parameter
	pub fn to_FlRange(&self, min: f64, max: f64) -> Blendable {
		match *self {
			FlRange(_,(min1,max1)) if min == min1 && max == max1 => *self,
			to_coerce => FlRange( to_coerce.to_FlUni().unwrap_FlUni() * (max - min) + min, (min, max) )
		}
	}

	// coerce to a Set of integers.  this requires a bit of subtlety.
	// we may want special behavior for coercing Sets to Sets with different ranges
	pub fn to_Set(&self, min: i64, max: i64) -> Blendable {
		match *self {
			Set(_,(min1,max1)) if min == min1 && max == max1 => *self,
			to_coerce => {
				let mut val = (to_coerce.to_FlUni().unwrap_FlUni() * ((max - min + 1) as f64)).floor() as i64;
				if val > max {
					val = max;
				}
				Set( val, (min,max) )
			}
		}
	}

	// if not a color, return white with intensity set
	pub fn to_ColorRGB(&self) -> Blendable {
		match *self {
			ColorRGB(_,_,_) => *self,
			to_coerce => {
				let mut val = (to_coerce.to_FlUni().unwrap_FlUni() * 256.).floor() as i64;
				if val > 255 {
					val = 255;
				}
				ColorRGB( val as u8, val as u8, val as u8 )
			}
		}
	}

	// unwrap as FlUni or task failure
	pub fn unwrap_FlUni(&self) -> f64 {
		match *self {
			FlUni(v) => v,
			_ => fail!() // task failure
		}
	}

	/*
	fn coerce_to(&self, coerce_to: Blendable) -> Blendable {
		coerce!(coerce_to to self over [FlUni,FlBi])
	}
	*/
}

// quick implementation of max blend for FlUni, fails for others
// TODO: complete this
fn blend_Max(top: Blendable, bot: Blendable) -> Blendable {
	match top {
		FlUni(tv) => FlUni(max!(tv, bot.to_FlUni().unwrap_FlUni())),
		_ => fail!()
	}
}


// quick implementation of mean blend for FlUni, fails for others
// TODO: complete this
fn blend_Mean(top: Blendable, bot: Blendable) -> Blendable {
	match top {
		FlUni(tv) => FlUni(tv * bot.to_FlUni().unwrap_FlUni() / 2.),
		_ => fail!()
	}
}

// all of the various blending modes
pub enum BlendMode {
	Clobber,
	Max,
	Min,
	AbsMax,
	AbsMin,
	AddClip,
	Mean,
	MultClip,
	MultRescale
	// etc.
}

//TODO: tests for all of these
