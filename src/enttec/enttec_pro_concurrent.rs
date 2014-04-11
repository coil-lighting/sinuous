//! Library defining the interface to the enttec pro usb dmx port.
//!
//! Functions for creating a port and sending data with it.  Opening and closing
//! are handled automatically by Rust.  Port will close upon deconstruction.
#![feature(globs)]
#![feature(phase)]
#[phase(syntax, link)] extern crate log;

use std::io;

use std::comm::Receiver;
use std::comm::TryRecvResult;
// really don't understand why I have to explicitly use all the variants of TryRecvResult
use std::comm::{Empty, Disconnected, Data};

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
	//println!("other commands: fps , univ_size");
	println!("dmx pattern options: same, rising, rainbow, strobe");
	println!("Command format:");
	println!("pat ampl period nframe wait_bet_frames_ms");
}

type DmxSender = Sender<~[u8]>;
type DmxReceiver = Receiver<~[u8]>;

// have to write this in a more roundabout way for it to work, as right now
// capturing mutable variables throws a compiler error which is known to be a bug
// see for instance https://github.com/mozilla/rust/issues/11958

// instance a new DMX port.  return a Sender to talk to the port
// we ought to have a way to ask the port to do things for us besides send
// implement this later
fn spawn_port(path: ~str) -> Result< DmxSender, SerialPortError > {

	// to check for init errors we start the port locally and then send it to another task
	let mut port = EnttecProOutPort::new(path);

	// try to start the port
	match port.start() {
		Ok(_) => (),
		Err(the_err) => return Err(the_err)
	}

	let port_imut = port;

	// make a channel to talk to the port
	let (tx, rx): (DmxSender, DmxReceiver) = channel();

	// spawn the new task for the DMX sender
	spawn(proc() {

		let mut taskport = port_imut;

		// make a local dmx buffer to write to
		let mut dmx: ~[u8];

		loop {

			// wait for a new packet
			match rx.flush() {
				Disconnected => {break;},
				// flush should never return Empty
				Empty => {unreachable!();},
				Data(d) => {dmx = d;}
			}

			// send the new packet
			match taskport.send(dmx.as_slice()) {
				Ok(_) => {},
				Err(an_err) => {debug!("Port send error: {:?}",an_err);}
			}
		}

	});

	Ok(tx)

}

// this is probably overkill but I thought it was a good experiment
// to ensure that the DMX frame we send is the most current, we need to make sure
// the value the port task receives is the newest.  So, flush the port and return
// the newest value.

// cannot implement a new method for a type defined elsewhere, so make a trait
trait Flush<T: Send> {
	fn flush(&self) -> TryRecvResult<T>;
}

// implement the Flush trait
impl<T: Send> Flush<T> for Receiver<T> {

	// ports are a queue.  we only want to send the newest value
	// so, flush the port and return only the most recent value or the lack thereof
	// note that this should NEVER return Empty
	// if this function returns Empty the task should fail!
	fn flush(&self) -> TryRecvResult<T> {

		// first see if there is data in the port at all
		match self.try_recv() {
			// if disconnected, say so
			Disconnected => { return Disconnected; }
			// if the port is empty, block until the port has data
			// note that if the port disconnects while we're here the task will fail
			Empty => { return Data(self.recv()); }
			// if the port has data, read until we reach the end and return last
			Data(d) => {

				// local buffer for most recent value
				let mut newest = d;

				loop {
					// keep receiving until empty or disconnceted
					match self.try_recv() {
						Disconnected => { return Disconnected; },
						// if empty, return the newest value
						Empty => { return Data(newest); }
						Data(newer) => { newest = newer; }
					}
				}
			}
		}
	}
}





fn main() {

	let dev = ~"/dev/tty.usbserial-EN077232";
	//let dev = ~"/Users/Chris/src/sinuous/src/enttec/test.txt";

	let port: DmxSender;

	match spawn_port(dev) {
		Ok(sender) => {port = sender;},
		Err(err) => {
			println!("{:?}",err);
			println!("Quitting.");
			return;
		}
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
		    /*
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
		    */
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


		    			port.send(dmx.clone());
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
