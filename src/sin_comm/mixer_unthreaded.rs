use sinuous_signal::*;
use blendable::*;
use ghetto_benchmarker::*;

pub struct MixerUnthreaded {
	channels: Vec<MixerChanUnthreaded>
}


impl MixerUnthreaded {
	fn mix(&mut self) -> Signal {

		let mut sig_out = new_sig();

		for chan in self.channels.iter() {
			sig_out = blend_sigs(chan.val.clone(),sig_out,chan.blend_mode);
		}

		sig_out
	}

	pub fn new(n_chan: uint) -> MixerUnthreaded {

		let mut chans = Vec::new();

		for _ in range(0,n_chan) {
			chans.push(MixerChanUnthreaded{val: new_sig(), blend_mode: Mean});
		}

		MixerUnthreaded{channels: chans}

	}

	pub fn mix_these(&mut self, vals: Vec<Signal>) -> Signal {
		self.set_chans(vals);
		self.mix()
	}

	// for testing unthreaded operation
	fn set_chans(&mut self, vals: Vec<Signal>) {
		for (mixchan, chanin) in self.channels.mut_iter().zip(vals.move_iter()) {
			mixchan.val = chanin;
		}
	}
}

struct MixerChanUnthreaded {
	val: Signal,
	blend_mode: BlendMode
}