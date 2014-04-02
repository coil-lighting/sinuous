//
//  EnttecDMXUSBProOutPort.m
//  LD50
//
//  Created by Michael Bissell on 3/24/06.
//  Copyright 2006 Michael Bissell. All rights reserved.
//

#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <sys/fcntl.h>
#include <sys/ioctl.h>
#include <errno.h>
#include <paths.h>
#include <sys/termios.h>
#include <sysexits.h>
#include <sys/param.h>
#include <sys/select.h>
#include <sys/time.h>
#include <time.h>
#include <math.h>
#include <IOKit/serial/ioss.h>  // for IOSSIOSPEED

#import "PFEnttecDMXUSBProOutPort.h"

@implementation PFEnttecDMXUSBProOutPort

- (void) dealloc {
	[_devicePath release];
	[super dealloc];
}

- (id) initWithDevicePath: (NSString*) dev {
    if( (self=[super init]) ) {
		// clear the dmx state
		bzero(_dmx,DMX_DATA_LEN); // clear the message buffer
		[self setDevicePath: dev];
		_settings.userSizeLSB        = 0;
		_settings.userSizeMSB        = 0;
		_settings.breakTime          = 9;
		_settings.markAfterBreakTime = 1;
		_settings.refreshRate        = 40;
		_settingsDirty               = YES;
		_registerCount               = DMX_LEN;
   }

	//NSLog(@"Done initializing %@ %@",[self class], dev);
    return self;
}

- (NSString*) devicePath {
	return _devicePath;
}

- (void) setDevicePath: (NSString*) dev {
	if(_devicePath != dev) {
		[_devicePath release]; // FIXME not perfectly thread-safe w.r.t. _threadStart. beware.
		_devicePath=[dev retain];
	}
}

- (unsigned char) getValueAtChannel:(unsigned)dmxChannel {
	return _dmx[dmxChannel+1];
}

- (void)sendDmx:(unsigned)dmxChannel byteValue:(unsigned char)value {
	_dmx[dmxChannel+1] = value;
}

// FIXME templatize this to factor out common code between this and DMXOutPort
- (void)sendDmx:(unsigned)dmxChannel floatValue:(float)value {
	if(dmxChannel >= DMX_LEN) {
		NSLog(@"sendDmx: DMX channel out of range. You said channel %i = %f",dmxChannel,value);
		return;
	}

	int intVal = (int)(255.9999*value); // FIXME test roundoffs

	// FIXME can't we do this by casting to unsigned char and skip the next test?

	// Clip the value in case dirty data leaks through
	if(intVal < 0)
		intVal = 0;
	else if(intVal > 255)
		intVal = 255;

	_dmx[dmxChannel+1] = intVal;
}

- (void) send {
	// TODO if we aren't going to use the threaded port, might as well merge PFUnthreaded...Port into this one
	// to simplify everything.
	[NSException raise: NSGenericException format: @"[%@ send] is a no-op. Subclass should impl this.",[self class]];
}

// render and send an array passed by python. OPTIMIZATION CANDIDATE.
- (void) renderFrame: (NSArray*) channels {
	// Optimization candidate. Use CFArray, CFNumber or NSDATA if possible for fast access.
	// ...or just grab C array out of channels?

	for(unsigned dmxChannel=0; dmxChannel<len; dmxChannel++) {
		NSNumber* level = [channels objectAtIndex: dmxChannel];
		unsigned char val = [level unsignedCharValue];
		_dmx[dmxChannel+1] = val;
	}
	if(_debug) {
		NSLog([self stateToString: _dmx]);
	}
	[self send];
}

- (void)sendDmxData:(bycopy NSData*)data {
	size_t length = MIN([data length], _registerCount);
	memcpy(_dmx, [data bytes], length);
}

- (void)sendDmx:(unsigned char*)values {
	memcpy(_dmx, values, _registerCount);
}

void sendData(int fd, unsigned char label, unsigned char * data, int length) {
	unsigned char _messageHeader[4];
	unsigned char _endOfMessage[1];
	_messageHeader[0]=0x7E;
	_messageHeader[1]=label;
	_messageHeader[2]=length&0xFF;
	_messageHeader[3]=length>>8;
	_endOfMessage[0]=0xE7;

	int ret=0;

	ret = write(fd, _messageHeader, 4);
    if(length!=0) {
		ret = write(fd, data, length);
	}
	ret = write(fd, _endOfMessage, 1);
	//ret = tcdrain(fd);
}

- (BOOL) isStarted {
	return _open;
}

- (BOOL) isReadyToStart {
	NSFileManager* fm = [NSFileManager defaultManager];
	return [fm fileExistsAtPath: _devicePath] && [fm isReadableFileAtPath: _devicePath];
}

- (void) start {
	[NSException raise: NSGenericException format: @"Unimplemented template method: [%@ start]", [self class]];
}

- (void) stop {
	[NSException raise: NSGenericException format: @"Unimplemented template method: [%@ stop]", [self class]];
}

 // in 10.67us units. range 9-127.
- (void)setBreakTime: (unsigned char) time {
	if(time < 9 || time > 127) {
		NSLog(@"Invalid break time: %i * 10.67us", time);
	} else {
		_settings.breakTime = time;
		_settingsDirty = YES;
	}
}

 // in 10.67us units. range 9-127.
- (void)setMarkAfterBreakTime: (unsigned char) time {
	if(time < 1 || time > 127) {
		NSLog(@"Invalid MAB time: %i * 10.67us", time);
	} else {
		_settings.markAfterBreakTime = time;
		_settingsDirty = YES;
	}
}

// USB device dmx refresh rate, in packets per second. range 0-40.
// 0 is special. It means "Go as fast as you can."
- (void)setRefreshRate: (unsigned char) rate {
	if(rate > 40) {
		NSLog(@"Invalid DMX refresh rate: %i fps", rate);
	} else {
		_settings.refreshRate = rate;
		_settingsDirty = YES;
	}
}

- (NSString*) description {
	return [NSString stringWithFormat: @"Enttec DMX USB Pro at %@", _devicePath];
}

// For formatting debug output. Slow!
NSMutableString* rjust(NSString* str,unsigned target_len) {
	unsigned str_len = [str length];
	NSMutableString* jstr = [[[NSMutableString alloc] init] autorelease];
	while([jstr length] + str_len < target_len) {
		[jstr appendString: @" "];
	}
	[jstr appendString: str];
	return jstr;
}

// For formatting debug output. Slow!
NSString* i2s(int i) {
	return [NSString stringWithFormat: @"%i", i];
}

- (NSString *) stateToString: (unsigned char*) buf {
	NSMutableString* state = [[[NSMutableString alloc] init] autorelease];
	[state appendString: @"\n"];

	for(unsigned x=0; x<16; x++) {
		[state appendString: rjust(i2s((32*x)+1),3)];
		[state appendString: @":"];
		for(unsigned y=0; y<32; y++) {
			unsigned c = (32*x)+y;
			[state appendString: rjust(i2s(buf[c]),4)];
		}
		[state appendString: @" ObjC\n"];
	}
	return state;
}

- (void) setRegisterCount: (unsigned) numberOfRegisters {
	_registerCount = numberOfRegisters;
}

- (unsigned int) registerCount {
	return _registerCount;
}

- (unsigned char) breakTime {
	return _settings.breakTime;
}

- (unsigned char) markAfterBreakTime {
	return _settings.markAfterBreakTime;
}

- (unsigned char) refreshRate {
	return _settings.refreshRate;
}

- (void) setDebug: (BOOL) debug {
	_debug = debug;
}

- (BOOL) debug {
	return _debug;
}

@end
