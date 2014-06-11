// TODO: figure out how to move test modules to a test subdirectory
extern crate std;
extern crate time;

use std::rc::Rc;
use std::cell::RefCell;
use time::*;

use device::*;
use dmx::*;
use effect::*;
use numeric::limit_unipolar_unit_f64_to_u8;
use render::*;
use topo::*;

/// Single-channel dimmer example.
/// Define the profile for a simple DMX dimmer. Make a universe and patch an
/// instance of a dimmer Device into this universe. Change its state and
/// measure performance.
pub fn create_dimmer() {
    let p = Profile{
        name: "Dimmer".to_string(),
        nickname: "Dim".to_string(),
        manufacturer: "Generic".to_string(),
        author: "Chris Macklin".to_string(),
        date: "June 7, 2014".to_string(),
        version: 0,
        chan_alloc: DmxChannelCount(1),
        root: Rc::new(Attr(Attribute {
            name: "Dimmer".to_string(),
            nickname: "Dim".to_string(),
            effect: (Dimmer, ColorspaceI, Value),
            topo: &'static continuous_euclidian_unipolar,
            default: Some(Continuous(0.)),
            dmx: Some(DmxMap{
                offset: DmxAddressOffsetSingle(0),
                renderer: DmxFloatRenderer(render_dmx_float_unipolar)
            })
        }))
    };

    let root_profile_branch = Rc::new(PBranch(ProfileBranch {
        name: "Device tree root".to_string(),
        nickname: "DevTrRt".to_string(),
        children: Vec::new(),
    }));

    let mut dev_tree_root = DeviceBranch{
        profile_branch: root_profile_branch.clone(),
        children: Vec::new(),
    };

    // TODO: factory method or constructor associated with DmxUniverse
    let univ = Rc::new(RefCell::new(DmxUniverse {
        id: 0,
        name: "U1".to_string(),
        frame: [0, ..512]
    }));

    let mut devices = Vec::new();

    for i in range(0u, 256u) {
        devices.push(patch(&p, &mut dev_tree_root, i, univ.clone()).unwrap());
    }

    fn write_dimmer_val(dim: &Device, v: f64) {
        match *dim.root {
            DeviceNodeEndpoint(ref e) => e.set_val(Continuous(v)),
            _ => ()
        }
    }

    let t0: f64 = precise_time_s();
    let n_trial = 10000;

    for n in range(0, n_trial) {
        let v = (n as f64)/(n_trial-1) as f64;

        // TODO: convert the for loop below to the equiv of python's enumerate(); don't use an extra uint i:
        let mut i: uint = 0;
        for d in devices.mut_iter() {
            write_dimmer_val(d,v);
            d.render();

            // TODO: fn, method or macro to deboilerplatify these contortions:
            let rendered_val = univ.deref().borrow().frame[i];
            assert!(rendered_val == limit_unipolar_unit_f64_to_u8(v));
            i = i + 1;
        }
        //println!("{:?}", univ.borrow().frame);
    }

    let avg_time = (1000f64 * (precise_time_s() - t0)) / (n_trial as f64);
    println!("Average time: {} ms", avg_time);

}

#[test]
pub fn test_dimmer() {
    create_dimmer();
}
