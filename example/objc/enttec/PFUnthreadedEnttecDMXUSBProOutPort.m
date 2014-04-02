//
//  PFUnthreadedEnttecDMXUSBProOutPort.m
//  LD50
//
//  Created by Michael Bissell on 8/3/06.
//  Copyright 2006 Michael Bissell. All rights reserved.
//

#import "PFUnthreadedEnttecDMXUSBProOutPort.h"
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/fcntl.h>
#include <sys/ioctl.h>
#include <errno.h>
#include <paths.h>
#include <sysexits.h>
#include <sys/param.h>
#include <sys/select.h>
#include <sys/time.h>
#include <time.h>
#include <math.h>
#include <IOKit/serial/ioss.h>  // for IOSSIOSPEED

@implementation PFUnthreadedEnttecDMXUSBProOutPort

- (id) initWithDevicePath: (NSString*) dev {
    if( (self=[super initWithDevicePath: dev]) ) {
		_fd = -1;
	}
	return self;
}

- (IBAction)start {

	if(_open) {
		if(_debug) NSLog(@"%@ at %@ is already open. Stopping the port and restarting.",[self class],_devicePath);
		[self stop];
	}
	
	if(![self isReadyToStart]) {
		NSLog(@"%@ at %@ is not ready to start.",[self class],_devicePath);
		return;
	}

	_settingsDirty = YES;
    const char *path = [_devicePath cStringUsingEncoding: NSASCIIStringEncoding];
    int ret = -1;
	_fd = -1;

	if(_debug) NSLog(@"Attempting to open %@ at %@.",[self class],_devicePath);

// trying O_NONBLOCK - works on intel, untested on G4
    if( path && (_fd=open(path, O_WRONLY | O_NOCTTY | O_NONBLOCK ))>=0 )

// without O_NONBLOCK: worked in PPC, hangs on G4:
//    if( path && (_fd = open(path, O_WRONLY | O_NOCTTY ))>=0 )


//    note: started getting kernel panicks from the 1.1 FTDI driver
//    after changing this to RDWR. Probably a coincidence. This is a known 1.1 bug.
//    if( path && (_fd=open(path, O_RDWR | O_NOCTTY ))>=0 )
    {
		_open = YES;
		if(_debug) NSLog(@"Opened %@ at %@.",[self class],_devicePath);

        if( ioctl(_fd, TIOCEXCL) == 0 ) {

            ret = tcgetattr(_fd, &oldOptions); // Save old options
            
            // set options
            speed_t speed = 250000; // speed is just a comment. it doesn't affect usb transfer speed.
            struct termios options = oldOptions;
            options.c_cflag = (CS8 | CSTOPB | CLOCAL | CREAD);
            options.c_lflag &= ~(ICANON | ECHO | ECHOE | ISIG);
            options.c_oflag &= ~OPOST;
            options.c_cc[ VMIN ] = 1;
            options.c_cc[ VTIME ] = 0;

			if(_debug) NSLog(@"Setting IO options.");
			ret = tcsetattr(_fd, TCSANOW, &options);
            ret = ioctl(_fd, IOSSIOSPEED, &speed);	
			// .. but it probably makes no difference to the Pro port.

            // clear tx
            ret = tcflush(_fd, TCIOFLUSH);

/*             probably not necessary*/
            // set RS485 for sending
            int flag;
            ret = ioctl(_fd, TIOCMGET, &flag);
            flag &= ~TIOCM_RTS;     // clear RTS flag
            ret = ioctl(_fd, TIOCMSET, &flag);

        } else {
			NSLog(@"FAILED setting term io options.");
			[self stop];
		}

    } else {
		NSLog(@"Couldn't open %@ '%@'",[self class],_devicePath);
		_fd = -1;
	}
}

- (IBAction)stop {
	if(_debug) NSLog(@"Stopping %@ %@...",[self class],_devicePath);
	if(_open) {
		int ret = tcdrain(_fd); // TODO check return values?
		ret = tcsetattr(_fd, TCSANOW, &oldOptions);
		close(_fd);
		_open = NO;
		_fd = -1;
	}
}

- (void) send {
	// send data buffer over DMX
	if( _open ) {
		// First, update the settings if they have changed.
		if(_settingsDirty) {
			if(_debug) NSLog(@"Sending settings.");
			sendData(_fd, setParametersMessageLabel, (unsigned char *) &_settings, sizeof(DMXUSBPROSetParamsType));
			/*int ret =*/ tcdrain(_fd); // TODO check return values?
			_settingsDirty = NO;
		}

		// The write appears to be non-blocking. (< ~250 microseconds to complete).
		sendData(_fd, outputOnlySendDmxMessageLabel, _dmx, _registerCount + 1); // length was DMX_DATA_LEN rather than _registerCount + 1 
	}
}

@end
