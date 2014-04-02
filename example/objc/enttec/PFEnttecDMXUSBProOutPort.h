//
//  EnttecDMXUSBProOutPort.h
//  LD50
//
//  Created by Michael Bissell on 3/24/06.
//  Copyright 2006 Michael Bissell. All rights reserved.
//

#import <Cocoa/Cocoa.h>
#import <Carbon/Carbon.h>
#import "PFDMXOutPort.h"

// 512 + start code = 513
#define DMX_DATA_LEN 513 
#define DMX_LEN 512 

#define startOfMessageDelim 0x7E
#define endOfMessageDelim 0xE7

#define reprogramFirmwareMessageLabel 1
#define programFlashPageMessageLabel 2
#define getParametersMessageLabel 3
#define setParametersMessageLabel 4
#define receivedDmxMessageLabel 5
#define outputOnlySendDmxMessageLabel 6
#define rdmSendDmxMessageLabel 7

typedef struct {
        unsigned char userSizeLSB;
        unsigned char userSizeMSB;
        unsigned char breakTime;
        unsigned char markAfterBreakTime;
        unsigned char refreshRate;
}DMXUSBPROSetParamsType;

@interface PFEnttecDMXUSBProOutPort : NSObject <PFSerialDMXOutPort> {

	unsigned _registerCount; 

	BOOL _open; // true when the port is open.
	BOOL _settingsDirty; // true when settings have changed and need to be transmitted to the usb dongle
	unsigned char _dmx[DMX_DATA_LEN]; // dmx buffer with start code 
	NSString* _devicePath;

	DMXUSBPROSetParamsType _settings;

	// tweaker mode not yet implemented.
	BOOL _chaseEnabled; // enable tweaker mode. NO for normal operation.
	BOOL _chaseForward; // NO for backward.

	BOOL _debug; // YES for logging
}

void sendData(int fd, unsigned char label, unsigned char * data, int length);

- (id) initWithDevicePath: (NSString*) dev;
- (void) setDevicePath: (NSString*) dev; // does not stop thread or close port before changing dev path. beware.
- (NSString*) devicePath;
- (void) start;
- (void) stop;
- (unsigned char) breakTime;
- (unsigned char) markAfterBreakTime;
- (unsigned char) refreshRate;
- (void) setBreakTime: (unsigned char) time; // in 10.67us units. range 9-127.
- (void) setMarkAfterBreakTime: (unsigned char) time; // in 10.67us units. range 1-127.
- (void) setRefreshRate: (unsigned char) rate; // USB device dmx refresh rate, in packets per second. range 1-40.
- (void) setDebug: (BOOL) debug;
- (BOOL) debug;
@end


