#![crate_id = "sinuous#0.01"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![feature(globs)]

//! Libsinuous provides and efficient, portable and flexible foundation library
//! for describing and controlling the parameters of DMX lights (especially
//! moving lights), mechanical effects, 2D motion graphics scenes, 3D models,
//! MIDI instruments, and audio plugin instruments such as VSTs and VSTis.
//!
//! Libsinuous seeks to abstract high-level aesthetic control from low-level
//! protocol mappings, and unlike most controllers, it happily allows the user
//! to build assemblies of off the shelf instruments and components into large
//! assemblies with coherently unified behaviors.
//!
//! This is the fourth generation of the LD50 project, which has been used in
//! various forms in events and theatrical productions for most of the past
//! decade.
//
// This is just a napkin sketch for the port of rbld50 core from Ruby to Rust.
// There is a LOT more to come. Fortunately the Ruby stuff mostly tests and runs
// okay.
//
// This is not a terribly well-organized, well-documented or well thought-out
// module just yet. It is our very first Rust program aside from FizzBuzz and
// Hello, World. Expect major reorganization.
//
// It compiles under the most recent 0.11-pre.

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


fn main() {
    let s = std::mem::size_of::<ProfileNode>();
	println!("{}", s);
    println!("{}", std::mem::size_of::<Attribute>());
    println!("{}", std::mem::size_of::<ProfileBranch>());
}


/*
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
*/