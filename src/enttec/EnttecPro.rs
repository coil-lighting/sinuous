#[feature(phase)];
#[phase(syntax, link)] extern crate log;
extern crate native;
use native::io::file::FileDesc;
use std::io::IoError;
use std::io;
use std::slice;
use std::c_str::CString;

use std::libc::{c_int,c_char};

// declare any static parameters
static DMX_LEN: uint = 512;

// import our wrappered C interface
#[link(name = "ioctrl")]
extern {
	fn open_port_file(path: *c_char) -> c_int;
	fn ioctrl_tiocexcl(fd: c_int) ->  c_int;
	fn tcgetattr(fildes: c_int, termios_p: *mut TermiosPtr) -> c_int;
	fn new_termios() -> *mut TermiosPtr;
	fn free_termios(to_free: *mut TermiosPtr);
	fn clone_termios(to_clone: *mut TermiosPtr) -> *mut TermiosPtr;
	fn tcsetattr_tcsanow(fd: c_int, options: *mut TermiosPtr) -> c_int;
	fn tcflush_io(fd: c_int) -> c_int;
	fn tcdrain(fd: c_int) -> c_int;
	fn ioctrl_tiocmgetandset(fd: c_int) -> c_int;
	fn set_options_enttec(options: *mut TermiosPtr);
}

enum TermiosPtr {}

// a Termios holds a pointer to the C struct
// Must never instantiate this except by using Termios::new() and others
struct Termios {
	target: *mut TermiosPtr
}

impl Termios {
	fn new() -> Termios {
		unsafe { Termios{target: new_termios()} }
	}

	fn set_as_enttec(&mut self) {
		unsafe { set_options_enttec(self.target); }
	}
}

// clone a Termios by calling the C function to allocate a new one and copy
impl Clone for Termios {
	fn clone(&self) -> Termios {
		unsafe { Termios{target: clone_termios(self.target)} }
	}
}

// go into C and call free
impl Drop for Termios {
	fn drop(&mut self) {
		unsafe { free_termios(self.target); }
	}
}

// "safe" interface to C functions

// open a port file using the C interface
fn open_file(path: &str) -> Option<FileDesc> {

	let fd = unsafe {open_port_file(path.to_c_str().unwrap()) };

	if fd >= 0 {
		Some(FileDesc::new(fd, true))
	}
	else {
		None
	}

}

// set the file to have exclusive access, check result for success
fn set_exclusive(file: &FileDesc) -> bool {
	let result = unsafe { ioctrl_tiocexcl(file.fd()) };

	if result == 0 {
		true
	}
	else {
		false
	}
}

// try to get the port options
fn get_port_options(file: &FileDesc) -> Option<Termios> {
	let options = Termios::new();
	let result = unsafe { tcgetattr(file.fd(), options.target) };
	// get the termios from the port

	// return options if successful
	if result == 0 {
		Some(options)
	}
	else {
		None
	}

}

// try and set the port options
fn set_port_options(file: &FileDesc, options: &Termios) -> bool {
	let result = unsafe { tcsetattr_tcsanow(file.fd(), options.target) };
	if result == 0 {
		true
	}
	else {
		false
	}
}

// flush the port; could return success or fail, but it wont fail if port is open
fn flush_port(file: &FileDesc) {
	unsafe { tcflush_io(file.fd()); }
}

// wait until the port has finished sending
fn drain_port(file: &FileDesc) {
	unsafe { tcdrain(file.fd()); }
}

// set rs485 for sending
// is this necessary?
fn set_rs485_for_sending(file: &FileDesc) {
	unsafe {ioctrl_tiocmgetandset(file.fd()); }
}



// enum for possible port errors
enum SerialPortError {
	UnspecifiedPortError,
	PortClosed,
	PortPathParseError,
	PortFileOpenError,
	PortSetExclusiveError,
	PortOptionsError,
	SendDataError(IoError)
}


// some enttec parameters
struct EnttecProParams {
        userSizeLSB: u8,
        userSizeMSB: u8,
        breakTime: u8,
        markAfterBreakTime: u8,
        refreshRate: u8
}

// to avoid relying on the memory layout of this struct, explicitly parse as a slice
impl EnttecProParams {
	fn as_vec(&self) -> ~[u8] {
		~[self.userSizeLSB, self.userSizeMSB, self.breakTime, self.markAfterBreakTime, self.refreshRate]
	}
}

// type that represents our interface to an enttec port
struct EnttecProOutPort {
	registerCount: uint,
	open: bool, // true when the port is open.
	settingsDirty: bool, // true when settings have changed and need to be transmitted to the usb dongle
	devicePath: ~str,
	settings: EnttecProParams,
	debug: bool, // YES for logging
	oldOptions: Termios, // stores old port settings. we restore them when we close the port.
	file: FileDesc // the file descriptor for the port
}

// ensure we close the port if it is open when we destruct
impl Drop for EnttecProOutPort {
	fn drop(&mut self) {
		if self.open {
			self.stop();
		}
	}
}


impl EnttecProOutPort {
 	// get rid of "init" concept from objC, in Rust constructors are usually new()
 	// do we want a generic constructor?
 	fn new(dev: ~str) -> EnttecProOutPort {

 		EnttecProOutPort{
 			registerCount: DMX_LEN,
 			open: false,
 			settingsDirty: true,
 			devicePath: dev,
 			settings: EnttecProParams{
 				userSizeLSB: 0,
 				userSizeMSB: 0,
 				breakTime: 9,
 				markAfterBreakTime: 1,
 				refreshRate: 40 },
 			debug: false,
 			oldOptions: Termios::new(),
 			// for now set file as a new FileDesc with fd = -1 and close_on_drop=false
 			file: FileDesc::new(-1, false)
 		}

 	}

 	// start the port.  return success or failure
 	fn start(&mut self) -> Result<(),SerialPortError> {
 		// if the port is open, stop it
 		if (self.open) {


			debug!("Port at {} is already open.  Stopping the port and restarting.",self.devicePath);
 			self.stop();
 		}


 		debug!("Attemping to open port at {}", self.devicePath);

 		/*
		let path_c: CString;

		// try to parse the devicePath as an actual path
		match Path::new_opt(self.devicePath.as_slice()) {
			Some(a_path) => { path_c = a_path.to_c_str();
			},
			None => {
				path_c = ("").to_c_str();
				return Err(PortPathParseError);
			}
		};

		debug!("Path parsed OK, will now call open().");

 		// attempt to open the file describing the port, write-only
 		match native::io::file::open(&path_c, std::io::Open, std::io::Write) {
 			Ok(the_file) => {

 				// settings are now changing
 				self.settingsDirty = true;
 				self.file = the_file; },
 			Err(the_error) => {
 				return Err(PortFileOpenError);
 			}
 		}
 		*/

 		match open_file(self.devicePath.as_slice()) {
 			Some(a_file) => {
 				self.settingsDirty = true;
 				self.file = a_file;
 			},
 			None => {
 				return Err(PortFileOpenError);
 			}
 		}

 		// if we made it this far, we opened the port file successfully

 		self.open = true;

		debug!("Opened port file at {} , will now attempt to configure",self.devicePath);

		// set the port to disallow any others to open it
		match set_exclusive(&self.file) {
			true => {},
			false => { //TODO: debug message here
				self.stop();
				return Err(PortSetExclusiveError);
			}
		}

		// try to retrieve the port options
		match get_port_options(&self.file) {
			Some(options) => { self.oldOptions = options; },
			None => {
				self.stop();
				return Err(PortOptionsError);
			}
		}

		let mut options = self.oldOptions.clone();


		/*

            options.c_cflag = (CS8 | CSTOPB | CLOCAL | CREAD);
            options.c_lflag &= ~(ICANON | ECHO | ECHOE | ISIG);
            options.c_oflag &= ~OPOST;
            options.c_cc[ VMIN ] = 1;
            options.c_cc[ VTIME ] = 0;
        */
        // this is all implemented in this method:
        options.set_as_enttec();

        debug!("Setting IO options.")

		match set_port_options(&self.file, &options) {
			true => {},
			false => { //TODO: debug message
				self.stop();
				return Err(PortOptionsError);
			}
		}


		// empty the port if there's something in there already
		flush_port(&self.file);

		/*
		// probably not necessary
            // set RS485 for sending
            int flag;
            ret = ioctl(_fd, TIOCMGET, &flag);
            flag &= ~TIOCM_RTS;     // clear RTS flag
            ret = ioctl(_fd, TIOCMSET, &flag);
        */
        // this is all implemented in this function:
        set_rs485_for_sending(&self.file);

        debug!("Port at {} is now ready for use.",self.devicePath);

        // we have successfully started the port
        Ok(())
    }

    // TODO: should this return Result?
    fn stop(&mut self) {

    	debug!("Stopping port at {}",self.devicePath);

    	if self.open {

    		// wait for the port to finish sending
    		drain_port(&self.file);

    		// set the options back to what they were originally
    		set_port_options(&self.file, &self.oldOptions);

    		// in Obj-C need to explicitly close self.file
    		// here if we reassign self.file, the old file will be dropped
    		// and closed automatically
    		self.file = FileDesc::new(-1, false);
    		self.open = false;


    	}
    }

    // why does the port store the dmx state internally?
    // seems like one should call a method port.send(theDMXValues)
    // I guess if the original implementation was threaded, and the IO are blocking operations,
    // then we may want setting DMX to be a different operation than sending it
    // either way, the pro port here should live in its own task
    // maybe we want internal storage to re-send in case of failure?
    // this ought to return success or failure
    // return a SerialPortError
    fn send(&mut self, dmx: &[u8]) -> Result<(),SerialPortError> {

    	if !self.open {
    		return Err(PortClosed);
    	}

    	// if the settings have changed, resend them
    	if self.settingsDirty {

    		debug!("Sending data on port {}",self.devicePath);

    		let settings_vec = self.settings.as_vec();

    		match send_data(&mut self.file, SetParameters, settings_vec.as_slice(), false ) {
    			Ok(_) => {},
    			Err(err_val) => {return Err(SendDataError(err_val));}
    		}
    		drain_port(&self.file);
    		self.settingsDirty = false;
    	}

    	// TODO: check if this is OK.
    	// original call was
    	// sendData(_fd, outputOnlySendDmxMessageLabel, _dmx, _registerCount + 1) // length was DMX_DATA_LEN rather than _registerCount + 1
    	match send_data(&mut self.file, OutputOnlySendDmx, dmx, true) {
    		Ok(_) => {},
    		Err(err_val) => {return Err(SendDataError(err_val));}
    	}

    	Ok(())

    }

	// in 10.67us units. range 9-127.
    fn set_break_time(&mut self, time: u8) {
    	if (time < 9 || time > 127) {
    		debug!("Invalid break time: {:u} * 10.67 us.", time);
    	}
    	else {
    		self.settingsDirty = true;
    		self.settings.breakTime = time;
    	}
    }

	// in 10.67us units. range 1-127.
    fn set_mark_after_break_time(&mut self, time: u8) {
    	if (time < 1 || time > 127) {
    		debug!("Invalid MAB time: {:u} * 10.67 us.", time);
    	}
    	else {
    		self.settingsDirty = true;
    		self.settings.markAfterBreakTime = time;
    	}
    }

    // USB device dmx refresh rate, in packets per second. range 0-40.
	// 0 is special. It means "Go as fast as you can."
    fn set_refresh_rate(&mut self, rate: u8) {
    	if rate > 40 {
    		debug!("Invalid DMX refresh rate: {:u} fps", rate);
    	}
    	else {
    		self.settings.refreshRate = rate;
    		self.settingsDirty = true;
    	}
    }

    fn description(&mut self) -> ~str {
    	(~"Enttec DMX USB Pro at ").append(self.devicePath.as_slice())
    }


}


// MessageLabel type
enum MessageLabel {
	ReprogramFirmware = 1u8,
	ProgramFlashPage = 2u8,
	GetParameters = 3u8,
	SetParameters = 4u8,
	ReceivedDmx = 5u8,
	OutputOnlySendDmx = 6u8,
	RdmSendDmx = 7u8
}


// basically a wrapper on several sequential write operations
// need to add the DMX start code before the DMX packet, thus the bool
// must send at least 24 DMX channels per frame for minimum time between breaks
fn send_data(file: &mut FileDesc, label: MessageLabel, data: &[u8], isDmx: bool) -> Result<(),IoError> {

	let header: ~[u8];

	// get the length
	let mut length = data.len();

	let mut pads_to_add: uint = 0;

	if isDmx {

		// add padding if length is less than 24
		if length < 24 {
			pads_to_add = 24 - length;
			length = 24;
		}

		header = ~[0x7E, label as u8, ((length+1) & 0xFF) as u8, (((length+1)>>8) & 0xFF) as u8, 0 ];
	}
	else {
		header = ~[0x7E, label as u8, (length & 0xFF) as u8, ((length>>8) & 0xFF) as u8 ];
	}

	let end_of_message: ~[u8] = ~[0xE7];

	match file.write(header) {
		Ok(_) => {},
		Err(err_val) => {return Err(err_val);}
	}

	if length != 0 {

		match file.write(data) {
			Ok(_) => {},
			Err(err_val) => {return Err(err_val);}
		}
	}

	if pads_to_add > 0 {
		match file.write(std::slice::from_elem(pads_to_add,0u8)) {
			Ok(_) => {},
			Err(err_val) => {return Err(err_val);}
		}
	}

	match file.write(end_of_message) {
		Ok(_) => {},
		Err(err_val) => {return Err(err_val);}
	}

	Ok(())
}

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

	let mut dmx: ~[u8] = ~[];

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
