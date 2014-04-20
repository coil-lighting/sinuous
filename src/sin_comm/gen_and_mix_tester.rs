
use mixer::*;
use mixer_unthreaded::*;
use sig_gen_tester::*;
use sinuous_signal::*;
use ghetto_benchmarker::*;

#[main]
fn main() {

	let n_gen = 2u;
	let n_chan = 1000;

	let mut trig_vec = Vec::new();
	let mut rec_vec = Vec::new();

	for _ in range(0,n_gen) {
		let (trig, rec) = SigGenTester::new_and_run(n_chan, ~"gen0");
		trig_vec.push(trig);
		rec_vec.push(rec);
	}

	let mixer_interface = Mixer::new_and_run(rec_vec);

	let n_frames = 100;

	let mut bm = GhettoBenchmarker::new();

	// run the generators and mixers
	for _ in range(0,n_frames) {

		bm.tic();
		for trig in trig_vec.iter() {
			trig.send(());
		}
		mixer_interface.trigger_input.send(());

		let mixer_out = mixer_interface.output.recv();
		bm.toc();

		sig_as_str(&mixer_out);

		//println!( "{}", sig_as_str(&mixer_out) );

	}

	println!("average framerate: {:?}", bm.ave_rate());

	bm.reset();

	let mut mixer_un = MixerUnthreaded::new(n_gen);

	let mut bm1 = GhettoBenchmarker::new();

	// now test the same parameters in a single-threaded implementation
	for _ in range(0,n_frames) {
		bm.tic();
		let mut sigs = Vec::new();
		for _ in range(0,n_gen) {
			sigs.push(SigGenTester::produce(n_chan, "gen0"));
		}

		bm1.tic();
		let mixer_out = mixer_un.mix_these(sigs);
		bm.toc();
		bm1.toc();

		sig_as_str(&mixer_out);
	}

	println!("average framerate: {:?}", bm.ave_rate());
	println!("average mixer framerate: {:?}", bm1.ave_rate());
}