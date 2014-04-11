//! Library defining the interface to the enttec pro usb dmx port.
//!
//! Functions for creating a port and sending data with it.  Opening and closing
//! are handled automatically by Rust.  Port will close upon deconstruction.
#![feature(globs)]
#![feature(phase)]
#[phase(syntax, link)] extern crate log;

use std::io;

use enttec_pro_port::*;

mod enttec_pro_port;

// rate is ticks per cycle
fn rainbow_stupid(tick: uint, amp: u8, period: uint) -> ~[u8] {
	let mut dmx: ~[u8] = ~[0, ..DMX_LEN];
	let arg: f64 = 2.*Float::pi()*(tick as f64) * (1./(period as f64));
	for chan in range(0,dmx.len()) {
		dmx[chan] = ((((arg + 2.*Float::pi()*((chan%3) as f64)/3.).sin() + 1.) /2. )*(amp as f64)) as u8;
	}

	dmx
}

// rate is DMX values per tick
fn all_rising(tick: uint, step_size: u8) -> ~[u8] {
	~[step_size * ((tick % 255) as u8), ..DMX_LEN]
}

fn all_same(val: u8) -> ~[u8] {
	~[val, ..DMX_LEN]
}

fn strobe(tick: uint, amp: u8) -> ~[u8] {
	all_same(amp * ((tick%2) as u8))
}

enum Pattern {
	Same,
	Rising,
	RainbowStupid,
	Strobe
}

fn print_info() {
	println!("type \"q\" to quit");
	println!("other commands: fps , univ_size");
	println!("dmx pattern options: same, rising, rainbow, strobe");
	println!("Command format:");
	println!("pat ampl period nframe wait_bet_frames_ms");
}

fn main() {

	let dev = ~"/dev/tty.usbserial-EN077232";
	//let dev = ~"/Users/Chris/src/sinuous/src/enttec/test.txt";

	let mut port = EnttecProOutPort::new(dev);
	match port.start() {
		Ok(_) => println!("port started successfully"),
		Err(the_err) => println!("{:?}",the_err)
	}


	print_info();

	let mut dmx: ~[u8];

	let mut pat = Same;
	let mut amp: u8 = 0;
	let mut rate: uint = 1;
	let mut n_frames: uint = 0;
	let mut wait: u64 = 1000;

	let mut univ_size: uint = 256;

	let mut quit = false;

	let mut set_fps = false;
	let mut set_univ_size = false;

	loop {
		for line in io::stdin().lines() {
			let line_conts = line.unwrap();
		    // print!("{}", read);

		    if line_conts == ~"q\n" {
		    	quit = true;
		    	break;
		    }
		    else if set_fps {
		    	let word = line_conts.words().next().unwrap();
		    	match from_str(word) {
		    		Some(f) => {
		    			port.set_refresh_rate(f);
		    			set_fps = false;
		    			print_info();
		    		},
		    		None => {
		    			println!("could not parse fps");
		    			set_fps = false;
		    		}
		    	}
		    }
		    else if set_univ_size {
		    	let word = line_conts.words().next().unwrap();
		    	let res: Option<uint> = from_str(word);
		    	match res {
		    		Some(n) if (n <= 256) => {
		    			univ_size = n;
		    			set_univ_size = false;
		    			print_info();
		    		},
		    		_ => {
		    			println!("could not parse universe size or out of bounds");
		    			set_univ_size = false;
		    		}
		    	}
		    }
		    else if line_conts == ~"fps\n" {
		    	println!("enter fps:");
		    	set_fps = true;
		    }
		    else if line_conts == ~"univ_size\n" {
		    	println!("enter universe size (0-256):");
		    	set_univ_size = true;
		    }
		    else {
		    	let words: ~[&str] = line_conts.words().collect();

		    	if words.len() < 5 {
		    		println!("Insufficient arguments.");
		    	}
		    	else {
			    	match words[0] {
			    		p if p == "same" => pat = Same,
			    		p if p == "rising" => pat = Rising,
			    		p if p == "rainbow" => pat = RainbowStupid,
			    		p if p == "strobe" => pat = Strobe,
			    		p => println!("Undefined pattern option: {}",p)
			    	}

			    	match from_str(words[1]) {
			    		Some(a) => amp = a,
			    		None => println!("amp parse error")
			    	}

			    	match from_str(words[2]) {
			    		Some(r) => rate = r,
			    		None => println!("Rate parse error")
			    	}

			    	match from_str(words[3]) {
			    		Some(n) => n_frames = n,
			    		None => println!("N frames parse error")
			    	}

			    	match from_str(words[4]) {
			    		Some(w) => wait = w,
			    		None => println!("ms wait parse error")
			    	}

			    	println!("{:?} {} {} {} {}", pat, amp, rate, n_frames, wait);

			    	for tick in range(0,n_frames) {
				    	match pat {
				    		Same => dmx = all_same(amp),
				    		Rising => dmx = all_rising(tick, rate as u8),
				    		RainbowStupid => dmx = rainbow_stupid(tick, amp, rate),
				    		Strobe => dmx = strobe(tick, amp)
				    	}

				    	// only send as many packets as we've defined the universe to be
		    			match port.send(dmx.slice_to(univ_size as uint)) {
							Ok(_) => (),//println!("port sent data successfully"),
							Err(the_err) => println!("{:?}",the_err)
						}

						std::io::timer::sleep(wait);

			    	}

			    	print_info();

		    	}




		    }

		}

		if quit {
			break;
		}
	}


}
