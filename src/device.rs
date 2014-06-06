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
pub enum AttributeValue {
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


pub struct Attribute {
    name: String, // e.g. "iris"

// IDEA: do away with dimensionality. make compound attributes just subtrees
// of device space. then we can always unbundle, say, an xy effect into an x
// and a y without having to write a separate model branch in the profile.
// also, it gives us the option to have heterogeneous topos and datatypes
// within a compound attribute.

    effect: (EffectType, EffectSubtype, EffectSubsubtype),
    topo: &'static Topo,
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
pub struct Profile {
    name: String,         // "Technobeam"
    nickname: String,     // "Techno"
    manufacturer: String, // "HES"
    author: String,       // "e.g. Steve Jobs"
    date: String,       // maybe we want to make this some kind of timestamp type
    version: int,       // 1, 2, 3...
    root: ProfileNode,  // No need to Box<> this, the profile owns its profile tree.
}

pub enum ProfileNode {
// TODO: REF PBranch and Attr into struct variants ... maybe only if we can prove the
// functions will be the same for both? we'll have to experiment.
    PBranch(ProfileBranch), // branch node
    Attr(Attribute), // leaf node
}

pub struct ProfileBranch {
    name: String, // "Technobeam"
    nickname: String, // "Techno"
    children: Vec<ProfileNode>,
}

pub enum Addr {
    DmxAddrType(DmxAddr), // TODO universe + address
    // Midi_addrType,
    // OscAddrType,
    // OpenPixelControlAddrType,
    // ...
}


// Use case: many physical technobeams all addressed to channel 1
pub struct DevicePatch {
    addr: Addr,

    // A DevicePatch has multiple locations in case more than one physical
    // device with the same address, is managed by one logical DevicePatch.
    locs: Vec<Loc>
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
pub struct DeviceBranch<'p> { // a DeviceBranch cannot outlive the profile it points to ('p)
    profile_branch: &'p ProfileBranch,
    children: Vec<DeviceNode<'p>>
}

impl<'p> DeviceBranch<'p> {
    pub fn render(&self, buffer: &mut[u8]) {
        // TODO verify that this operates by reference, not by copy!
        for child in self.children.iter() {
            match *child {
                // Rust manual: "Patterns that bind variables default to binding
                // to a copy or move of the matched value (depending on the
                // matched value's type). This can be changed to bind to a
                // reference by using the 'ref' keyword, or to a mutable
                // reference using 'ref mut'."
                DeviceNodeBranch(ref d) => d.render(buffer),
                DeviceNodeEndpoint(ref d) => d.render(buffer)
            };
        }
    }
}

pub struct DeviceEndpoint<'p> { // a DeviceEndpoint cannot outlive the profile it points to ('p)
    attribute: &'p Attribute,
    value: Option<AttributeValue>, // required if rendering is implemented for this attribute
}

impl<'p> DeviceEndpoint<'p> {
    pub fn render(&self, buffer: &mut[u8]) {

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
        match dmx.renderer {
            DmxFloatRenderer(r) => {
                r(nf, offset, buffer);
            },
            DmxFloatBipolarWithRangeRenderer(r, ref range) => {
                r(nf, range, offset, buffer);
            },
            DmxFloatUnipolarWithRangeRenderer(r, ref range) => {
                r(nf, range, offset, buffer);
            },
            DmxDoubleRenderer(r) => {
                r(nf, offset, buffer);
            },
            DmxIntIndexedWithRangeRenderer(r, ref range) => {
                r(ni, range, offset, buffer);
            },
            DmxBooleanWithRangeRenderer(r, ref range) => {
                // TODO do away with booleans and use int
                let bi: bool = match ni {
                    0 => false,
                    _ => true
                };
                r(bi, range, offset, buffer);
            },
            DmxSpinBipolar2ChWithRangeRenderer(r, ref range) => {
                r(nf, range, offset, buffer);
            },
        };
    }
}

// A device is an actual instance of a device. A device is described by its
// Profile tree.
// CSM: we probably want to replace these essentially placeholder types by
// declaring DeviceBranch and DeviceEndpoint as struct variants of DeviceNode.
pub enum DeviceNode<'p> {
    DeviceNodeBranch(DeviceBranch<'p>),
    DeviceNodeEndpoint(DeviceEndpoint<'p>),
}

pub struct Device<'p> {
    profile: &'p Profile,
    name: String,
    nickname: String, // shorter, to save space (defaults to name, truncated)

    // we probably want to define a type to contain this information to help ease
    // the job of the device patcher later
    id: uint,
    // A Device has multiple addrs in case one logical device manages more than
    // one distinct device address. (See DevicePatch.locs for the case where
    // multiple physical devices all share the same address.)

    // we may consider using the optimized type SmallVector, optimized for cases when
    // the length is almost always 0 or 1
    // syntax::util::small_vector::SmallVector
    patches: Vec<DevicePatch>,
    root: DeviceNode<'p>,
}

impl<'p> Device<'p> {
    pub fn render(&mut self) {
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
        for patch in self.patches.mut_iter() {
            match patch.addr {
                DmxAddrType(ref mut dmx_addr) => {

                    // Why this extra song and dance around the universe reference?
                    // The Universe is wrapped in a RefCell, so we need to try to
                    // get a mutable reference to the contents, which could fail.
                    // Also, the RefMut that is returned determines the lifetime of
                    // the &mut we get from it, so we need to hold it in this scope.
                    match dmx_addr.try_get_univ_ref() {
                        Some(mut u_ref) => {
                            let buffer = dmx_addr.slice_universe(&mut u_ref);

                            match self.root {
                                DeviceNodeBranch(ref d) => d.render(buffer),
                                DeviceNodeEndpoint(ref d) => d.render(buffer)
                            };

                        },
                        None => () // if something else is already writing to
                        // the universe buffer, we can't get access.  give up.
                    }
                    /*
                    let (buffer, univ_ref)  = dmx_addr.slice_universe();

                    // TODO do not render redundantly if patched more than once (with the same protocol)?
                    // TODO? track most recent profile slice as we descend the tree
                    // FUTURE think about how subprofiles might come in handy (where a
                    // profile gives its attributes relative addresses to relative
                    // addresses, writing slices on slices)
                    match *self.root {
                        DeviceNodeBranch(ref d) => d.render(buffer),
                        DeviceNodeEndpoint(ref d) => d.render(buffer)
                    };
                    */
                }
            }
        }
    }
}
