#![macro_escape]

// there are really two kinds of messages in OSC, they are either a message or a bundle
#[deriving(Show,Clone,Eq,Ord)]
pub enum OscPacket {
	// a message is the real meat
	OscMessage{
		pub addr: ~str,
		pub args: Vec<OscArg>
	},
	// a bundle is intended to synchronize multiple commands
	// essentially it bundles together multiple OSC packets
	OscBundle{
		pub time_tag: OscTimeTag,
		pub conts: Vec<OscPacket>
	}
}

// enum to contain the allowed OSC argument types
// this is the 1.0 specification, may want to look into 1.1 in the future
#[deriving(Show,Clone,Eq,Ord)]
pub enum OscArg {
	OscInt(i32),
	OscFloat(f32),
	OscStr(~str),
	OscBlob(Vec<u8>)
	/* OSC 1.1 stuff
	OscInt64(i64),
	OscFloat64(f64),
	OscTime(OscTimeTag),
	OscSymbol(~str),
	OscChar(Ascii),
	OscColor((u8, u8, u8, u8)),
	OscMidi(MidiMessage),
	OscAssert(OscAssertion),
	OscArray(Vec<OscArg>)
	*/
}

// some placeholders for possible eventual OSC 1.1 support
/*
enum OscAssertion {
	True,
	False,
	Nil,
	Infinitum
}

struct MidiMessage {
	port_id: u8,
	status_byte: u8,
	data1: u8,
	data2: u8
}
*/

/// Convert an OscArg to its correpsonding type tag character
pub fn arg_to_type_tag(arg: &OscArg) -> char {
	match *arg {
		OscInt(_) => 'i',
		OscFloat(_) => 'f',
		OscStr(_) => 's',
		OscBlob(_) => 'b'
		/*
		OscInt64(_) => 'h',
		OscFloat64(_) => 'd',
		OscTime(_) => 't',
		OscSymbol(_) => 'S',
		OscChar(_) => 'c',
		OscColor(_) => 'r',
		OscMidi(_) => 'm',
		OscAssert(a) => {
			match a {
				True => 'T',
				False => 'F',
				Nil => 'N',
				Infinitum => 'I'
			}
		},
		// this was all nice and pretty until OscArray had to come fuck it all up
		// with OscArray I have to return a damn string instead of a char.  lame.
		// this right here is enough reason to just support OSC 1.0 for now
		OscArray(v) =>
		*/
	}
}

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

pub type OscTimeTag = (u32, u32);