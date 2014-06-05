use collections::hashmap::*;

use std::hash::sip::*;

use blendable::*;

#[deriving(Hash,Eq,TotalEq,Clone)]
pub struct SigDesc {
	pub desc: ~str,
	pub id: u64
}

pub type Signal = HashMap<SigDesc, Blendable, SipHasher>;

// Sadly, can't use static methods on typedefs
// may want to set capacity here, default is 32
pub fn new_sig() -> Signal {
	HashMap::new()
}

pub fn new_sig_with_cap(cap: uint) -> Signal {
	HashMap::with_capacity(cap)
}

// blend two signals together
// bottom layer is blended into the top layer, consuming the top
// returns the mutated bottom layer
// this makes sense for mixing up from the bottom layer of a mixer
// have to iterate over one signal and check the other for collisions
// collisions are blended
// non-collisions are inserted into top
// lots of room for optimization here:
// 		iterate over smaller of the two for collision check
//		maybe tricks with capacities?
pub fn blend_sigs(top: Signal, mut bot: Signal, mode: BlendMode) -> Signal {

	// iterate over the entries in the top signal layer
	for (desc_top,val_top) in top.move_iter() {
		// for each entry, insert into bottom if not present or blend if collision
		match bot.find_copy(&desc_top) {
			Some(val_bot) => {
				bot.insert(desc_top, val_top.blend_over(val_bot, mode));
			},
			None => {
				bot.insert(desc_top, val_top);
			}
		}
	}

	bot
}

pub fn sig_as_str(sig: &Signal) -> ~str {
	let mut output = ~"Signal contents:\n";
	for (desc,val) in sig.iter() {
		output.push_str(format!("{:?} ",*desc));
		output.push_str(format!("{:?}\n",*val));
	}
	output
}

// define types for channels for sending and receiving these
pub type SignalReceiver = Receiver<Signal>;
pub type SignalSender = Sender<Signal>;