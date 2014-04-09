//
//  PFDMXOutPort.h
//  LD50
//
//  Created by Michael Bissell on 5/31/06.
//  Copyright 2006 Michael Bissell. All rights reserved.
//

//#import <PuppeteerFrameworkUtil/PuppeteerFrameworkUtil.h>

@protocol PFDMXOutPort <NSObject>  // NSObject is the root object class in objC
// return a formatted log of this port's dmx state
- (NSString *) stateToString: (unsigned char*) buf;

// Suboptimal, but easy to invoke across the bridge: (channels is NSArray of NSNumber)
- (void) renderFrame: (NSArray*) channels;

// return a formatted log of this port's device path, register count, and dmx state
- (NSString*) description;
- (void) sendDmx: (unsigned) dmxChannel floatValue: (float) value;
- (void) sendDmx: (unsigned) dmxChannel byteValue: (unsigned char) value;
- (void) setRegisterCount: (unsigned) numberOfRegisters;
- (unsigned) registerCount;
- (unsigned char) refreshRate;
- (void) setRefreshRate: (unsigned char) rate;

// If these don't apply to a particular port type, they should noOp.
- (void) stop;
- (void) start;
- (BOOL) isStarted;
- (BOOL) isReadyToStart;
@end

@protocol PFSerialDMXOutPort <PFDMXOutPort>
- (id) initWithDevicePath: (NSString*) dev;
- (void) setDevicePath: (NSString*) dev; // does not stop thread or close port before changing dev path. beware.
- (NSString*) devicePath;
@end

/* not possible with pro port
@protocol PFChasingPort <NSObject>
- (void) setChase: (id<ChaseProvider>) chaseProvider;
- (void) setChaseEnabled: (BOOL) enabled;
- (BOOL) chaseEnabled;
- (void) setChaseForward: (BOOL) forward;
- (BOOL) chaseForward;
@end
*/

@protocol PFSender
- (void) send; // out ports may optionall implement this to indicate they support send-on-demand
@end