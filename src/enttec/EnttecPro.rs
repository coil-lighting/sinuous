extern crate native;
use native::io::file::FileDesc;
use std::io::IoError;
use std::c_str::CString;


// declare any static parameters
static DMX_LEN: uint = 512;
static DMX_DATA_LEN: uint = 513;

// TODO: WTF do we do about this
struct Termios;

// TODO: implement Clone for Termios
impl Clone for Termios {
	fn clone(&self) -> Termios {
		Termios
	}
}

// TODO: implement some/all of these guys as Traits
// could have Trait POSIXSerial
// EntttecPro itself is a Trait that inherits from POSIXSerial
// then these functions could be methods instead of sitting out here

// TODO: create this function
// this function should be a wrapper on ioctl(_fd, TIOCEXCL)
// takes a FileDesc on which to perform this operation
// returns Ok if sucessful, Err(int) if otherwise
fn set_exclusive(file: &FileDesc) -> Result<(),int> {
	Ok(())
}

// TODO: create this function
// tcgetattr(_fd, &oldOptions);
// presenting a different interface
// give a FileDesc and get a Termios back or an error
fn get_port_options(file: &FileDesc) -> Result<Termios,int> {
	Ok(Termios)
}

// TODO: create this function
// int ret = tcsetattr(_fd, TCSANOW, &options);
// give a FileDesc and the port options
// return Ok or Err(int)
// Do we need to return Result?  can this fail?
fn set_port_options(file: &FileDesc, options: Termios) -> Result<(),int> {
	Ok(())
}

// TODO: create this function
// int ret = tcflush(_fd, TCIOFLUSH);
// give a FileDesc
// return a Result?  no return?
fn flush_port(file: &FileDesc) {
}

// TODO: create this function
// int ret = tcdrain(_fd); // TODO check return values?
// give a FileDesc
// return a Result?
fn drain_port(file: &FileDesc) {
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


// some parameters; should we make this static?
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
			Ok(_) => {},
			Err(err_val) => { //TODO: debug message here
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

		// TODO: ensure that this is a deep copy
		let mut options = self.oldOptions.clone();

		// TODO: implement these settings once we know what Termios will be
		/*

            options.c_cflag = (CS8 | CSTOPB | CLOCAL | CREAD);
            options.c_lflag &= ~(ICANON | ECHO | ECHOE | ISIG);
            options.c_oflag &= ~OPOST;
            options.c_cc[ VMIN ] = 1;
            options.c_cc[ VTIME ] = 0;
        */

        // TODO: debug message
		// if(_debug) NSLog(@"Setting IO options.");
		match set_port_options(&self.file, options) {
			Ok(_) => {},
			Err(err_val) => { //TODO: debug message
				self.stop();
				return Err(PortOptionsError);
			}
		}



		// TODO: match on error condition?
		flush_port(&self.file);

		/* TODO: what are we doing with this section?
		// probably not necessary
            // set RS485 for sending
            int flag;
            ret = ioctl(_fd, TIOCMGET, &flag);
            flag &= ~TIOCM_RTS;     // clear RTS flag
            ret = ioctl(_fd, TIOCMSET, &flag);
            */

        // we have successfully started the port
        Ok(())
    }

    // TODO: should this return Result?
    fn stop(&mut self) {
    	// TODO: debug message
    	// if(_debug) NSLog(@"Stopping %@ %@...",[self class],_devicePath);

    	if self.open {

    		// TODO: match on return value?
    		// what does this do?
    		drain_port(&self.file);

    		// TODO: what about this guy?
    		// ret = tcsetattr(_fd, TCSANOW, &oldOptions);

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

}
