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
        root: Rc::new(RefCell::new(ProfileGraphAttribute(Attribute {
            name: "Dimmer".to_string(),
            nickname: "Dim".to_string(),
            effect: (Dimmer, ColorspaceI, Value),
            topo: &'static continuous_euclidian_unipolar,
            default: Some(Continuous(0.)),
            dmx: Some(DmxMap{
                offset: DmxAddressOffsetSingle(0),
                renderer: DmxFloatRenderer(render_dmx_float_unipolar)
            })
        })))
    };

    let dev_tree_root = Rc::new(RefCell::new(DeviceBranch{
        profile_branch: None,
        children: Vec::new(),
    }));

    // TODO: factory method or constructor associated with DmxUniverse
    let univ = Rc::new(RefCell::new(DmxUniverse {
        id: 0,
        name: "U1".to_string(),
        frame: [0, ..512]
    }));

    let mut devices = Vec::new();

    // Patch 256 dimmers
    let device_ct: uint = 512;
    for i in range(0u, device_ct) {
        devices.push(patch(&p, dev_tree_root.clone(), i, univ.clone()).unwrap());
    }

    fn write_dimmer_val(dim: &Device, v: f64) {
        match *dim.root.borrow() {
            DeviceTreeEndpoint(ref e) => e.set_val(Continuous(v)),
            _ => ()
        }
    }

    let t0: f64 = precise_time_s();
    let trial_ct = 100000;

    let mut write_ct: uint = 0;
    for n in range(0, trial_ct) {
        let v = (n as f64)/(trial_ct-1) as f64;

        // TODO: convert the for loop below to the equiv of python's enumerate(); don't use an extra uint i:
        let mut i: uint = 0;
        for d in devices.mut_iter() {
            write_dimmer_val(d,v);
            d.render();

            // This assertion adds about +25% to the runtime of this test:
            // TODO: fn, method or macro to deboilerplatify these contortions:
            let rendered_val = univ.borrow().frame[i];
            assert!(rendered_val == limit_unipolar_unit_f64_to_u8(v));
            i = i + 1;
        }
        write_ct = write_ct + i
        //println!("{:?}", univ.borrow().frame);
    }

    let total_ms = 1000f64 * (precise_time_s() - t0);
    let avg_ms_per_universe = total_ms / (trial_ct as f64);
    let avg_ms_per_channel = total_ms / (write_ct as f64);
    let writes_per_dmx_frame = 20f64 / avg_ms_per_channel;
    let universes_per_dmx_frame = writes_per_dmx_frame / 512f64;

    println!("For {} iterations:", trial_ct);
    println!("    Average time per {}-channel test iteration: {} milliseconds",
        device_ct, avg_ms_per_universe);
    println!("    Average time per channel write: {} picoseconds for each of {} writes",
        avg_ms_per_channel * 1000000f64, write_ct)
    println!("    Extrapolated throughput: ({} channels = {} universes) per 20ms standard DMX frame",
        writes_per_dmx_frame as u64, universes_per_dmx_frame as u64);
}

#[test]
pub fn test_dimmer() {
    create_dimmer();
}
