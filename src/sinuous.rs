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
extern crate time;

use test_dimmer::*; // TODO: figure out how to move test modules to a subdirectory

mod blend;
mod device;
mod dmx;
mod effect;
mod numeric;
mod range;
mod render;
mod test_dimmer;
mod topo;
mod world;


fn main() {
    create_dimmer();
}
