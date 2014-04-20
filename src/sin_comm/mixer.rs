use sinuous_signal::*;
use blendable::*;
use trigger::*;
use ghetto_benchmarker::*;

pub struct MixerInterface {
	pub trigger_input: TriggerSender,
	pub output: SignalReceiver
}

pub struct Mixer {
	channels: Vec<MixerChan>,
	trigger: TriggerReceiver,
	output: SignalSender
}


impl Mixer {
	fn mix(&mut self) -> Signal {

		let mut sig_out = new_sig();

		for chan in self.channels.iter() {
			sig_out = blend_sigs(chan.val.clone(),sig_out,chan.blend_mode);
		}

		sig_out
	}

	pub fn new_and_run(chan_inputs: Vec<SignalReceiver>) -> MixerInterface {

		let (trig_tx, trig_rx): (TriggerSender, TriggerReceiver) = channel();
		let (out_tx, out_rx): (SignalSender, SignalReceiver) = channel();

		let mut mixer = Mixer{channels: Vec::new(), trigger: trig_rx, output: out_tx};

		for input in chan_inputs.move_iter() {

			mixer.channels.push( MixerChan{has_input: true, input: input, val: new_sig(), blend_mode: Mean} )
		}

		spawn(proc() {
			let mut run_mixer = mixer;
			run_mixer.run();
		});

		MixerInterface{trigger_input: trig_tx, output: out_rx}

	}

	fn run(&mut self) {

		let mut sig_out: Signal;

		let mut bm = GhettoBenchmarker::new();

		loop {
			// wait for a trigger signal or kill if disconnected
			match self.trigger.recv_opt() {
				Some(_) => {},
				None => {break;}
			}

			// wait for each mixer channel to receive an update
			for chan in self.channels.mut_iter() {
				chan.recv();
			}
			bm.tic();
			sig_out = self.mix();
			bm.toc();

			self.output.send(sig_out);
		}

		println!("{:?}",bm.ave_rate());
	}
}

struct MixerChan {
	has_input: bool,
	input: SignalReceiver,
	val: Signal,
	blend_mode: BlendMode
}

impl MixerChan {
	fn recv(&mut self) {
		if self.has_input {
			self.val = self.input.recv();
		}
	}
}