#![macro_escape]

/// calculate how many more bytes we need to make the argument a multiple of four
pub fn four_byte_pad(len: uint) -> uint {
	let rem = len % 4;
	match rem {
		0 => 0,
		v => 4 - v
	}
}


// helper macro for this commonly-needed operation
macro_rules! pad_with_null(
	($operator:ident $operation:ident $n:expr) => (
		for _ in range(0,four_byte_pad($n)) {
			$operator.$operation('\0');
		}
	)
)


// experimental macro for making dealing with osc args easier
/// Helper macro to check if an OscArg is a given type, produces a bool
#[macro_export]
macro_rules! arg_is_type(
	($arg:ident, $targ_var:ident) => (
		match $arg {
			$targ_var(_) => true,
			_ => false
		}
	)
)


// experimental macro for making dealing with osc args easier
/// Helper macro to unwrap an OscArg as a given type, produces None if the types don't match
#[macro_export]
macro_rules! unwrap_if(
	($arg:ident is $kind:ident) => (
		match $arg {
			$kind(v) => Some(v),
			_ => None
		}
	)
)