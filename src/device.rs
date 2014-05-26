//! Models describing device profiles and concrete devices (as trees).
use dmx::DmxAddr;
use dmx::DmxMap;
use dmx::DmxAddressOffsetSingle;
use effect::EffectType;
use effect::EffectSubtype;
use effect::EffectSubsubtype;
use render::DmxFloatRenderer;
use render::DmxFloatBipolarWithRangeRenderer;
use render::DmxFloatUnipolarWithRangeRenderer;
use render::DmxDoubleRenderer;
use render::DmxIntIndexedWithRangeRenderer;
use render::DmxBooleanWithRangeRenderer;
use render::DmxSpinBipolar2ChWithRangeRenderer;

use topo::Topo;
use world::Loc;

// Named subtypes for the primitive storage representing the numeric value for
// a Device Attribute's instance.
enum AttributeValue {
    Continuous(f64),
    Discrete(i64), // TODO - decide whether to make this unsigned instead
}


// I experimentally eliminated dimensionality in order to simplify device
// modeling. A (sub)Profile/sub(Device)'s dimensionality is just the
// number of its children. This allows us to break down an xy cartesian
// coordinate into separate x and y effects; we'll just have to decide
// how to indicate that we want to blend in higher dimensions, i.e. blend
// the xy branch, not x and y separately, for blend modes where x and y
// are independent. This might be tricky, but it's no trickier than trying
// to work around the atomicity of an xy 2D attribute for cases where I
// really just want to deal with x. - MB

// notes from rb about labeling indexed ranges:
    // TODO - ONLY IF THIS ATTRIBUTE NEEDS LABELED INDICES
    //     @indexTable=nil // if required, a tuple of strings, one per mode, in order
// ...but shouldn't index labels be in with the range struct?

// Here's another idea from ruby. It's no longer clear how this logic will
// pan out. Maybe we'll need to resurrect this idea because modal logic can
// be arbitrarily complex on board the device, but hopefully we can express
// modes through the layout of the device tree instead:
    // ONLY IF THIS ATTRIBUTE GOVERNS A MODAL CLUSTER
    //     @clusterMethod=nil

struct Attribute {
    name: ~str, // e.g. "iris"
    effect: (EffectType, EffectSubtype, EffectSubsubtype),
    topo: Box<Topo>,
    default: Option<AttributeValue>, // required if rendering is implemented
    dmx: Option<DmxMap>, // required if DMX rendering is implemented
}


// Leftover AttributeTypes from the old DMX renderers, now refactored:
    // // name                     *dmx* renderer (old Ruby style map)
    // ModalParent          ,// => LrenderDMXModalParent,
    // IndexVirtual         ,// => LrenderDMXVirtual,
    // Cluster              ,// => LrenderDMXCluster,


// Hypothesis: devices' descriptions are trees of ProfileElements, and this will
// suffice to describe everything from simple, 1 dimensional, nonmodal
// attributes like a dimmer channel to complex, multidimensional, modal
// attribute clusters such as mspeed-smoothed continuous litho wheel angle.
struct Profile {
    name: ~str,         // "Technobeam"
    nickname: ~str,     // "Techno"
    manufacturer: ~str, // "HES"
    author: ~str,       // "e.g. Steve Jobs"
    version: int,       // 1, 2, 3...
    root: Box<ProfileNode>,
}

enum ProfileNode {
    PBranch(ProfileBranch), // branch node
    Attr(Attribute), // leaf node
}

struct ProfileBranch {
    name: ~str, // "Technobeam"
    nickname: ~str, // "Techno"
    children: ~[ProfileNode],
}

enum Addr {
    DmxAddrType(DmxAddr), // TODO universe + address
    // Midi_addrType,
    // OscAddrType,
    // OpenPixelControlAddrType,
    // ...
}

// TODO better name
struct DevicePatch {
    addr: Addr,

    // A DevicePatch has multiple locations in case more than one physical
    // device with the same address, is managed by one logical DevicePatch.
    locs: ~[Loc]
}

// There seem to be three layers of deviceness:
//
// 1) A 'physical' device, or rather a specific instance, i.e. one ani_mated model
// in an external editor or one fixture on a tree.
//
// 2) An 'output' device (for lack of a better name), which is an addressed
// device on a specific universe. Whether we want to allow multiple output ports
// per universe is a separate question. It would allow us to make a soft DMX
// splitter without complicating the first-order output mappings associated with
// devices in the scenegraph. This might be called a 'logical device' or something.
// Or a 'patched device.' (Perhaps a universe gets patched, too.)
//     - these are managed by DevicePatches.
//
// 3) A logical device which is part of the scenegraph. Maybe 'SceneDevice'?

// TODO - optional custom labels for each node? currently just default to profile node labels
struct DeviceBranch {
    profile_branch: Box<ProfileBranch>,
    children: ~[DeviceNode]
}

impl DeviceBranch {
    fn render(&self, buffer: &mut[u8]) {
        // TODO verify that this operates by reference, not by copy!
        for child in self.children.iter() {
            match child {
                // Rust manual: "Patterns that bind variables default to binding
                // to a copy or move of the matched value (depending on the
                // matched value's type). This can be changed to bind to a
                // reference by using the 'ref' keyword, or to a mutable
                // reference using 'ref mut'."
                &DeviceNodeBranch(ref d) => d.render(buffer),
                &DeviceNodeEndpoint(ref d) => d.render(buffer)
            };
        }
    }
}

struct DeviceEndpoint {
    attribute: Box<Attribute>,
    value: Option<AttributeValue>, // required if rendering is implemented for this attribute
}

impl DeviceEndpoint {
    fn render(&self, buffer: &mut[u8]) {

        let n: AttributeValue = match self.value {
            Some(v) => v,
            None => match self.attribute.default {
                Some(d) => d,
                None => fail!("Every attribute which supports rendering must supply a default value.")
            }
        };

        // TODO: also make sure this doesn't copy anything by value
        let dmx: &DmxMap = match self.attribute.dmx {
            Some(ref x) => x,
            None => fail!("Every attribute which supports rendering must supply a DmxMap (for now).")
        };

        let offset: uint = match dmx.offset {
            // TODO Might just collapse this field into DmxAttributeRenderers.
            DmxAddressOffsetSingle(i) => i
        };

        // TODO support for renderers requiring multiple offsets, such as
        // a double renderer that does not write to adjacent indices in the
        // framebuffer.

        let (nf, ni) = match n {
            Continuous(c) => (c, 0),
            Discrete(d) => (0.0, d)
        };

        // Adapt to the interface of the renderer in question.
        match &dmx.renderer {
            &DmxFloatRenderer(r) => {
                r(nf, offset, buffer);
            },
            &DmxFloatBipolarWithRangeRenderer(r, ref range) => {
                r(nf, range, offset, buffer);
            },
            &DmxFloatUnipolarWithRangeRenderer(r, ref range) => {
                r(nf, range, offset, buffer);
            },
            &DmxDoubleRenderer(r) => {
                r(nf, offset, buffer);
            },
            &DmxIntIndexedWithRangeRenderer(r, ref range) => {
                r(ni, range, offset, buffer);
            },
            &DmxBooleanWithRangeRenderer(r, ref range) => {
                // Maybe do away with booleans and use ints?
                let bi: bool = match ni {
                    0 => false,
                    _ => true
                };
                r(bi, range, offset, buffer);
            },
            &DmxSpinBipolar2ChWithRangeRenderer(r, ref range) => {
                r(nf, range, offset, buffer);
            },
        };
    }
}

// A device is an actual instance of a device. A device is described by its
// Profile tree.
enum DeviceNode {
    DeviceNodeBranch(DeviceBranch),
    DeviceNodeEndpoint(DeviceEndpoint),
}

struct Device {
    profile: Profile,
    name: ~str,
    nickname: ~str, // shorter, to save space (defaults to name, truncated)

    // A Device has multiple addrs in case one logical device manages more than
    // one distinct device address. (See DevicePatch.locs for the case where
    // multiple physical devices all share the same address.)
    patches: ~[DevicePatch],
    root: Box<DeviceNode>,
}

impl Device {
    fn render(&self) {
        // Proposed: Assemble a list of slices, each a view on a universe's dmx
        // framebuffer, each slice aligned with the beginning of the device and
        // only as long as the device, to isolate damage.
        //
        // Question: how to aggregate multiple devices, spread out over the
        // whole universe, into a single quasi-device:
        //
        // Answer 1: just use a parent node in the device tree?
        // Answer 2: Return a buffer that looks like a mut slice, but is
        // actually mapped via a lookup table?
        //
        // Limitation: under this proposal, an individual attribute can't
        // corrupt other (non-overlapping) devices, but it also can't range
        // over the whole universe without belonging to a device that claims
        // the whole universe. Hopefully this is okay.
        for patch in self.patches.iter() {
            match &patch.addr {
                &DmxAddrType(addr) => {
                    let buffer: &mut [u8] = addr.slice_universe();

                    // TODO do not render redundantly if patched more than once (with the same protocol)?
                    // TODO? track most recent profile slice as we descend the tree
                    // FUTURE think about how subprofiles might come in handy (where a
                    // profile gives its attributes relative addresses to relative
                    // addresses, writing slices on slices)
                    match *self.root {
                        DeviceNodeBranch(ref d) => d.render(buffer),
                        DeviceNodeEndpoint(ref d) => d.render(buffer)
                    };
                }
            }
        }
    }
}
