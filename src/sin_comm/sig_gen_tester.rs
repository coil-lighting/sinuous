use trigger::*;
use sinuous_signal::*;
use blendable::*;

pub struct SigGenTester {
	output: SignalSender,
	trigger: TriggerReceiver
}

impl SigGenTester {
	// for present purposes we can just make one function that does it all
	// really we should pass in the trigger source and the data source
	pub fn new_and_run(n: u64, label: ~str) -> (TriggerSender, SignalReceiver) {
		let (sig_tx, sig_rx): (SignalSender, SignalReceiver) = channel();
		let (trig_tx, trig_rx): (TriggerSender, TriggerReceiver) = channel();
		let gen = SigGenTester{output: sig_tx, trigger: trig_rx};

		spawn(proc() {
			gen.run(n,label);
		});

		(trig_tx, sig_rx)
	}

	fn run(&self, n: u64, label: &str) {
		loop {
			// wait for a trigger signal or kill if disconnected
			match self.trigger.recv_opt() {
				Some(_) => {},
				None => {break;}
			}

			self.output.send(SigGenTester::produce(n, label));
		}
	}

	pub fn produce(n: u64, label: &str) -> Signal {
		let mut sig = new_sig_with_cap(n as uint);
		for i in range(0,n) {
			sig.insert(SigDesc{desc: label.to_owned(), id: i}, FlUni((i as f64) / (n as f64)));
		}
		sig
	}
}