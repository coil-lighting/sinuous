//! Library defining the interface to the enttec pro usb dmx port.
//!
//! Functions for creating a port and sending data with it.  Opening and closing
//! are handled automatically by Rust.  Port will close upon deconstruction.
#[feature(globs)];
#[feature(phase)];
#[phase(syntax, link)] extern crate log;

extern crate std;

extern crate native;

use native::io::file::FileDesc;
use std::io::IoError;

use enttec_pro_port::posix_port::*;

mod posix_port;

// declare any static parameters
pub static DMX_LEN: uint = 512;

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
pub struct EnttecProOutPort {
	priv open: bool, // true when the port is open.
	priv settingsDirty: bool, // true when settings have changed and need to be transmitted to the usb dongle
	priv devicePath: ~str,
	priv settings: EnttecProParams,
	priv oldOptions: Termios, // stores old port settings. we restore them when we close the port.
	priv file: FileDesc // the file descriptor for the port
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
 	// do we want the port to always start when constructed?
 	pub fn new(dev: ~str) -> EnttecProOutPort {

 		EnttecProOutPort{
 			open: false,
 			settingsDirty: true,
 			devicePath: dev,
 			settings: EnttecProParams{
 				userSizeLSB: 0,
 				userSizeMSB: 0,
 				breakTime: 9,
 				markAfterBreakTime: 1,
 				refreshRate: 40 },
 			oldOptions: Termios::new(),
 			// for now set file as a new FileDesc with fd = -1 and close_on_drop=false
 			file: FileDesc::new(-1, false)
 		}

 	}

 	// start the port.  return success or failure
 	pub fn start(&mut self) -> Result<(),SerialPortError> {
 		// if the port is open, stop it
 		if self.open {


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

    // stop the port and restore original settings
    pub fn stop(&mut self) {

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

    // method to send a DMX packet using the port
    pub fn send(&mut self, dmx: &[u8]) -> Result<(),SerialPortError> {

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

    	match send_data(&mut self.file, OutputOnlySendDmx, dmx, true) {
    		Ok(_) => {},
    		Err(err_val) => {return Err(SendDataError(err_val));}
    	}

    	Ok(())

    }

    // we may want to make these functions private later
	// in 10.67us units. range 9-127.
    pub fn set_break_time(&mut self, time: u8) {
    	if (time < 9 || time > 127) {
    		debug!("Invalid break time: {:u} * 10.67 us.", time);
    	}
    	else {
    		self.settingsDirty = true;
    		self.settings.breakTime = time;
    	}
    }

	// in 10.67us units. range 1-127.
    pub fn set_mark_after_break_time(&mut self, time: u8) {
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
    pub fn set_refresh_rate(&mut self, rate: u8) {
    	if rate > 40 {
    		debug!("Invalid DMX refresh rate: {:u} fps", rate);
    	}
    	else {
    		self.settings.refreshRate = rate;
    		self.settingsDirty = true;
    	}
    }

}


// MessageLabel type
// Right now I have commented out all variants that we don't use
enum MessageLabel {
	//ReprogramFirmware = 1u8,
	//ProgramFlashPage = 2u8,
	//GetParameters = 3u8,
	SetParameters = 4u8,
	//ReceivedDmx = 5u8,
	OutputOnlySendDmx = 6u8
	//RdmSendDmx = 7u8
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