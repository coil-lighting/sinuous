extern crate native;
use native::io::file::FileDesc;
use std::io::IoError;
use std::c_str::CString;

use std::libc::{c_int};


// declare any static parameters
static DMX_LEN: uint = 512;
static DMX_DATA_LEN: uint = 513;

// import our wrappered C interface
#[link(name = "ioctrl_wrapper")]
extern {
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
	target: *TermiosPtr
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
	fn clone(&mut self) -> Termios {
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

// set the file to have exclusive access, check result for success
fn set_exclusive(file: &FileDesc) -> bool {
	unsafe { let result: int = ioctrl_tiocexcl(file.fd()); }

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

	// get the termios from the port
	unsafe { let result = tcgetattr(file.fd(), options.target); }

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
	unsafe { let result = tcsetattr_tcsanow(file.fd(), options.target) -> c_int; }
	it result == 0 {
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
	unsage { tcdrain(file.fd()); }
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
 			oldOptions: Termios,
 			// for now set file as a new FileDesc with fd = -1 and close_on_drop=false
 			file: FileDesc::new(-1, false)
 		}

 	}

 	// start the port.  return success or failure
 	fn start(&mut self) -> Result<(),SerialPortError> {
 		// if the port is open, stop it
 		if (self.open) {
 			if self.debug {
 				//TODO: debug message here
 				// NSLog(@"%@ at %@ is already open. Stopping the port and restarting.",[self class],_devicePath);
 			}
 			self.stop();
 		}




	    // TODO: debug message here
		// if(_debug) NSLog(@"Attempting to open %@ at %@.",[self class],_devicePath);

		let path_c: CString;

		// try to parse the devicePath as an actual path
		match Path::new_opt(self.devicePath.as_slice()) {
			Some(a_path) => { path_c = a_path.to_c_str();
			},
			None => { //TODO: debug message here: path parse failure
				path_c = ("").to_c_str(); // this probably wont compile
				return Err(PortPathParseError);
			}
		};


 		// attempt to open the file describing the port, write-only
 		match native::io::file::open(&path_c, std::io::Open, std::io::Write) {
 			Ok(the_file) => {

 				// settings are now changing
 				self.settingsDirty = true;
 				self.file = the_file; },
 			Err(the_error) => { //TODO: debug message about error syndrome
 				return Err(PortFileOpenError);
 			}
 		}

 		// if we made it this far, we opened the port file successfully

 		self.open = true;
 		// TODO: debug message here
		// if(_debug) NSLog(@"Opened %@ at %@.",[self class],_devicePath);

		// set the port to disallow any others to open it
		match set_exclusive(&self.file) {
			true => {},
			false => { //TODO: debug message here
				// NSLog(@"FAILED setting term io options.");
				self.stop();
				return Err(PortSetExclusiveError);
			}
		}

		// try to retrieve the port options
		match get_port_options(&self.file) {
			Ok(options) => { self.oldOptions = options; },
			Err(err_val) => { // TODO: debug message of error syndrome
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

        // TODO: debug message
		// if(_debug) NSLog(@"Setting IO options.");
		match set_port_options(&self.file, options) {
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

        // we have successfully started the port
        Ok(())
    }

    // TODO: should this return Result?
    fn stop(&mut self) {
    	// TODO: debug message
    	// if(_debug) NSLog(@"Stopping %@ %@...",[self class],_devicePath);

    	if self.open {

    		// wait for the port to finish sending
    		drain_port(&self.file);

    		// set the options back to what they were originally
    		set_port_options(&self.file, &self.oldOptions)

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
    		// TODO: debug message, port not open
    		return Err(PortClosed);
    	}

    	// if the settings have changed, resend them
    	if self.settingsDirty {
    		// TODO: debug message here
    		match send_data(&mut self.file, SetParameters, self.settings.as_vec().as_slice() ) {
    			Ok(_) => {},
    			Err(err_val) => {return Err(SendDataError(err_val));}
    		}
    		drain_port(&self.file);
    		self.settingsDirty = false;
    	}

    	// TODO: check if this is OK.
    	// original call was
    	// sendData(_fd, outputOnlySendDmxMessageLabel, _dmx, _registerCount + 1) // length was DMX_DATA_LEN rather than _registerCount + 1
		// may need to change send_data to include length; is registerCount + 1 anything besides length of _dmx?
    	match send_data(&mut self.file, OutputOnlySendDmx, dmx) {
    		Ok(_) => {},
    		Err(err_val) => {return Err(SendDataError(err_val));}
    	}

    	Ok(())

    }

	// in 10.67us units. range 9-127.
    fn set_break_time(&mut self, time: u8) {
    	if (time < 9 || time > 127) {
    		// TODO: debug message here
    		// NSLog(@"Invalid break time: %i * 10.67us", time);
    	}
    	else {
    		self.settingsDirty = true;
    		self.settings.breakTime = time;
    	}
    }

	// in 10.67us units. range 1-127.
    fn set_mark_after_break_time(&mut self, time: u8) {
    	if (time < 1 || time > 127) {
    		// TODO debug message here
    		// NSLog(@"Invalid MAB time: %i * 10.67us", time);
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
    		// TODO: debug message here
    		// NSLog(@"Invalid DMX refresh rate: %i fps", rate);
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
fn send_data(file: &mut FileDesc, label: MessageLabel, data: &[u8]) -> Result<(),IoError> {
	let length = data.len();
	let header: ~[u8] = ~[0x7E, label as u8, (length & 0xFF) as u8, (length>>8) as u8 ];
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

	match file.write(end_of_message) {
		Ok(_) => {},
		Err(err_val) => {return Err(err_val);}
	}

	Ok(())
}

fn main() {

	let mut port = EnttecProOutPort::new(~"/Users/Chris/src/sinuous/src/enttec/test_file.txt");
	match port.start() {
		Ok(_) => println!("fake port started successfully"),
		Err(the_err) => println!("{:?}",the_err)
	}

	let test_payload = ~[123u8, ..10];

	match port.send(test_payload.as_slice()) {
		Ok(_) => println!("fake port sent data successfully"),
		Err(the_err) => println!("{:?}",the_err)
	}

	port.stop();

}
