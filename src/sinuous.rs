#![crate_id = "sinuous#0.01"]
#![allow(dead_code)]
#![allow(non_camel_case_types)]

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
    println!("Done.")
}
