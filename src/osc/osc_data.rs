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
}

/// Convert an OscArg to its correpsonding type tag character
pub fn arg_to_type_tag(arg: &OscArg) -> char {
	match *arg {
		OscInt(_) => 'i',
		OscFloat(_) => 'f',
		OscStr(_) => 's',
		OscBlob(_) => 'b'
	}
}

pub type OscTimeTag = (u32, u32);