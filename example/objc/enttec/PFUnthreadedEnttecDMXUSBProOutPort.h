//
//  PFUnthreadedEnttecDMXUSBProOutPort.h
//  LD50
//
//  Created by Michael Bissell on 8/3/06.
//  Copyright 2006 Michael Bissell. All rights reserved.
//

#import <Cocoa/Cocoa.h>
#import <Carbon/Carbon.h>
#import "PFEnttecDMXUSBProOutPort.h"
#include <sys/termios.h>

@interface PFUnthreadedEnttecDMXUSBProOutPort : PFEnttecDMXUSBProOutPort <PFSender> {
	struct termios oldOptions; // stores old port settings. we restore them when we close the port.
    int _fd; // file descriptor index
}

@end
