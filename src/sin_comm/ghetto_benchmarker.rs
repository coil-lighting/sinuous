use time::*;

pub struct GhettoBenchmarker {
	t0: f64,
	t1: f64,
	cycles: uint,
	total_time: f64
}

impl GhettoBenchmarker {
	pub fn new() -> GhettoBenchmarker {
		GhettoBenchmarker{t0: 0., t1: 0., cycles: 0, total_time: 0.}
	}

	pub fn tic(&mut self) {
		self.t0 = precise_time_s();
	}

	pub fn toc(&mut self) {
		self.t1 = precise_time_s();
		self.total_time += self.t1 - self.t0;
		self.cycles += 1;
	}

	pub fn ave_time(&mut self) -> f64 {
		self.total_time / self.cycles as f64
	}

	pub fn ave_rate(&mut self) -> f64 {
		1f64 / (self.ave_time())
	}

	pub fn reset(&mut self) {
		self.t0 = 0.;
		self.t1 = 0.;
		self.cycles = 0;
		self.total_time = 0.;
	}
}