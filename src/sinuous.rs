#![crate_id = "sinuous#0.01"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(globs)]

//! Nude pencil drawings of basic libsinuous constructs.
//
// This is just a napkin sketch for the port of rbld50 core from Ruby to Rust.
// There is a LOT more to come. Fortunately the Ruby stuff mostly tests and runs
// okay.
//
// This is not a terribly well-organized, well-documented or well thought-out
// module just yet. It is only my very first Rust program since FuzzBuzz and
// Hello, World. Expect major reorganization.
//
// It compiled under 0.11-pre.
//
// # Comments that look like this are just copied verbatim from the Ruby version.
// # They do not necessarily pertain to this Rust version. Just for reference
// # while I try to port this puppy.

use blend::*;
use device::*;
use dmx::*;
use effect::*;
use numeric::*;
use range::*;
use render::*;
use topo::*;
use world::*;


mod blend;
mod device;
mod dmx;
mod effect;
mod numeric;
mod range;
mod render;
mod topo;
mod world;

/*
fn main() {
	println!("done.");
}
*/


fn main() {

	let mut univ = DmxUniverse {
	    id: 0, // TEMP
	    name: "test".to_owned(),
	    frame: [0, ..512] // TODO: REF to Vec<u8>, no box says CM
	};


	let dimmer = Device {
    	profile: Profile {
    		name: box "Dimmer".to_owned(),         // "Technobeam"
    		nickname: "Dim".to_owned(),     // "Techno"
    		manufacturer: "Generic".to_owned(), // "HES"
    		author: "Chris".to_owned(),       // "e.g. Steve Jobs"
    		version: 0,       // 1, 2, 3...
    		root: box Attr(Attribute {
    			name: "Dimmer".to_owned(), // e.g. "iris"
			    effect: (Dimmer, ColorspaceI, Value),
			    topo: box continuous_euclidian_unipolar,
			    default: Some(Continuous(0)), // required if rendering is implemented
			    dmx: Some(DmxMap{offset: 0, renderer: DmxFloatRenderer(renderDmxFloat)}), // required if DMX rendering is implemented
			}),
		},
    	name: "Dimmer1".to_owned(),
    	nickname: "Dim1".to_owned(), // shorter, to save space (defaults to name, truncated)

	    // A Device has multiple addrs in case one logical device manages more than
	    // one distinct device address. (See DevicePatch.locs for the case where
	    // multiple physical devices all share the same address.)
    	patches: ~[ DevicePatch{
    		addr: DmxAddrType(DmxAddr{
    			universe: &mut univ,
    			address: 0,
    			length: 1
    		}),
    		locs: ~[Loc{
    			position: Position{x:0, y:0, z:0},
    			orientation: Orientation{pan: 0, tilt: 0, roll:0}
    		}]
    	}],
    	root: box DeviceNodeEndpoint(DeviceEndpoint {
    		attribute: box Attribute {
    			name: "Dimmer".to_owned(), // e.g. "iris"
			    effect: (Dimmer, ColorspaceI, Value),
			    topo: box continuous_euclidian_unipolar,
			    default: Some(Continuous(0)), // required if rendering is implemented
			    dmx: Some(DmxMap{offset: 0, renderer: DmxFloatRenderer(renderDmxFloat)}), // required if DMX rendering is implemented
			},
    		value: Some(Continuous(0)), // required if rendering is implemented for this attribute
		})
	};
}
