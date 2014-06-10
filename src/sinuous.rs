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

extern crate debug;

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

    let p = Profile{
        name: "Dimmer".to_string(),
        nickname: "Dim".to_string(),
        manufacturer: "Generic".to_string(),
        author: "Chris Macklin".to_string(),
        date: "June 7, 2014".to_string(),
        version: 0,
        chan_alloc: DmxSingleton(1),
        root: Attr(Attribute{
            name: "Dimmer".to_string(),
            effect: (Dimmer, ColorspaceI, Value),
            topo: &'static continuous_euclidian_unipolar,
            default: Some(Continuous(0.)),
            dmx: Some(DmxMap{
                offset: DmxAddressOffsetSingle(0),
                renderer: DmxFloatRenderer(render_dmx_float_unipolar)
            })
        })
    };

    // every device branch must refer to a profile branch.  this doesn't make sense
    // for the root of the device tree.  might want to consider special-casing this,
    // or it may just not matter.
    let dummy_pbranch = ProfileBranch{
        name: "Device tree root".to_string(),
        nickname: "DevTrRt".to_string(),
        children: Vec::new()
    };

    let mut dev_tree_root = DeviceBranch{
        profile_branch: &dummy_pbranch,
        children: Vec::new()
    };

    // construct our horrible universe monstrosity
    let univ = std::rc::Rc::new(std::cell::RefCell::new(DmxUniverse{id: 0, name: "U1".to_string(), frame: [0, ..512]}));

    let mut my_dimmer = patch(&p, &mut dev_tree_root, 0, univ.clone()).unwrap();

    // now I've patched a device in the universe.  write a value to it and render!
    match *my_dimmer.root {
        DeviceNodeEndpoint(ref e) => e.set_val(Continuous(0.5)),
        _ => ()
    }

    my_dimmer.render();

    println!("{:?}", univ.borrow().frame);

    match *my_dimmer.root {
        DeviceNodeEndpoint(ref e) => e.set_val(Continuous(0.5)),
        _ => ()
    }

    my_dimmer.render();

    println!("{:?}", univ.borrow().frame);

    // sadly, patching another device of course fails because we already have a mutable pointer to the root of the device tree.
    /*
    let mut my_dimmer = patch(&p, &mut dev_tree_root, 1, univ.clone()).unwrap();
    */
}
