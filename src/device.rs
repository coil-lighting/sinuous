//! Models describing device profiles and concrete devices (as trees).

// I experimentally eliminated dimensionality in order to simplify device
// modeling. A (sub)Profile/sub(Device)'s dimensionality is just the
// number of its children. This allows us to break down an xy cartesian
// coordinate into separate x and y effects; we'll just have to decide
// how to indicate that we want to blend in higher dimensions, i.e. blend
// the xy branch, not x and y separately, for blend modes where x and y
// are independent. This might be tricky, but it's no trickier than trying
// to work around the atomicity of an xy 2D attribute for cases where I
// really just want to deal with x. - MB

use dmx::DmxAddr;
use dmx::DmxMap;
use dmx::DmxAddressOffsetSingle;
use dmx::DmxUniverse;
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

use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

// Named subtypes for the primitive storage representing the numeric value for
// a Device Attribute's instance.
#[deriving(Clone)]
pub enum AttributeValue {
    Continuous(f64),
    Discrete(i64), // TODO - decide whether to make this unsigned instead
}

pub struct Attribute {
    pub name: String, // e.g. "Gobo wheel 1 position"
    pub nickname: String, // e.g. "Gobo1"
    pub effect: (EffectType, EffectSubtype, EffectSubsubtype),
    pub topo: &'static Topo,
    pub default: Option<AttributeValue>, // required if rendering is implemented
    pub dmx: Option<DmxMap>, // required if DMX rendering is implemented
}


// Hypothesis: devices' descriptions are trees of ProfileElements, and this will
// suffice to describe everything from simple, 1 dimensional, nonmodal
// attributes like a dimmer channel to complex, multidimensional, modal
// attribute clusters such as mspeed-smoothed continuous litho wheel angle.
pub struct Profile {
    pub name: String,         // "Technobeam"
    pub nickname: String,     // "Techno"
    pub manufacturer: String, // "HES"
    pub author: String,       // "e.g. Steve Jobs"
    pub date: String,       // maybe we want to make this some kind of timestamp type
    pub version: int,       // 1, 2, 3...
    pub chan_alloc: ChannelAlloc, // what kinds of addresses do we need to allocate to patch one?

    pub root: Rc<ProfileNode>,
}

pub enum ChannelAlloc {
    DmxSingleton(uint)
}

pub enum ProfileNode {
    PBranch(ProfileBranch), // branch node
    Attr(Attribute), // leaf node
}

pub struct ProfileBranch {
    pub name: String, // "Technobeam"
    pub nickname: String, // "Techno"
    pub children: Vec<Rc<ProfileNode>>,
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
    pub addr: Addr,

    // A DevicePatch has multiple locations in case more than one physical
    // device with the same address, is managed by one logical DevicePatch.
    pub locs: Vec<Loc>
}

impl DevicePatch {

    /// Make a new patch in the given universe. Do not (yet) check for
    /// conflicting patches. (TODO: *optionally* check for conflicts.)
    /// The returned patch has no associated Locs.
    pub fn new_dmx(addr: uint, len: uint, univ: Rc<RefCell<DmxUniverse>> ) -> DevicePatch {
        DevicePatch{
            addr: DmxAddrType(DmxAddr{
                universe: univ,
                address: addr,
                length: len
            }),
            locs: Vec::new()
        }
    }
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
pub struct DeviceBranch { // a DeviceBranch cannot outlive the profile it points to ('p)
    pub profile_branch: Rc<ProfileNode>, // We actually want Rc<ProfileBranch>, but haven't gotten the compiler to accept that it's a special case of Rc<ProfileNode>
    pub children: Vec<Rc<DeviceNode>>,
}

impl DeviceBranch {
    pub fn render(&self, buffer: &mut[u8]) {
        // TODO verify that this operates by reference, not by copy!
        for child in self.children.iter() {
            match **child {
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

pub struct DeviceEndpoint { // a DeviceEndpoint cannot outlive the profile it points to ('p)
    pub attribute: Rc<ProfileNode>, // We actually want Rc<Attribute>, but haven't gotten the compiler to accept that it's a special case of Rc<ProfileNode>
    pub value: Cell<Option<AttributeValue>>, // required if rendering is implemented for this attribute
}

impl DeviceEndpoint {

    pub fn get_val(&self) -> Option<AttributeValue> {
        self.value.get()
    }

    // TODO: check to make sure that the attribute value has the right type for
    // the Attribute.
    pub fn set_val(&self, val: AttributeValue) {
        self.value.set(Some(val))
    }

    pub fn get_attribute<'a>(&'a self) -> &'a Attribute {
        match *self.attribute {
            Attr(ref a) => a,
            _ => fail!("Every DeviceEndpoint must be bound to an Attr(Attribute), not a PBranch(ProfileBranch) ProfileNode."),
        }
    }

    /// Fail if you have corrupted the device tree by linking an endpoint to
    /// a non-Attribute ProfileBranch.
    pub fn render(&self, buffer: &mut[u8]) {
        let attribute = self.get_attribute();

        // Either return my value, or return default if no value.
        let n: AttributeValue = match self.get_val() {
            Some(v) => v,
            None => match attribute.default {
                Some(d) => d,
                None => fail!("Every attribute which supports rendering must supply a default value."),
            }
        };

        // TODO: also make sure this doesn't copy anything by value
        let dmx: &DmxMap = match attribute.dmx {
            Some(ref x) => x,
            None => fail!("Every attribute which supports rendering must supply a DmxMap (for now)."),
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
pub enum DeviceNode {
    DeviceNodeBranch(DeviceBranch),
    DeviceNodeEndpoint(DeviceEndpoint),
}

pub struct Device<'p> {
    pub profile: &'p Profile, // multiple similar devices hold an immutable reference to each Profile
    pub name: String,
    pub nickname: String, // shorter, to save space on screen (defaults to name, truncated)

    // we probably want to define a type to contain this information to help ease
    // the job of the device patcher later
    pub id: uint,
    // A Device has multiple addrs in case one logical device manages more than
    // one distinct device address. (See DevicePatch.locs for the case where
    // multiple physical devices all share the same address.)

    // we may consider using the optimized type SmallVector, optimized for cases when
    // the length is almost always 0 or 1
    // syntax::util::small_vector::SmallVector
    pub patches: Vec<DevicePatch>,
    pub root: Rc<DeviceNode>,
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

                            //self.root.render(buffer);
                            // Device.root is now always a DeviceBranch

                            match *self.root {
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

pub fn device_subtree_from_profile_subtree(root: &Rc<ProfileNode>) -> Rc<DeviceNode> {
    match **root { // Rc implements the trait deref, so the * operator works.
        // if this is a profile branch, recurse over it
        PBranch(ref pb) => {
            Rc::new(DeviceNodeBranch(DeviceBranch {
                profile_branch: root.clone(),

                // recurse over all of the children and collect into a vector
                children: pb.children.iter().map(|pb_child| device_subtree_from_profile_subtree(pb_child)).collect()
            }))
        },
        // If this is a leaf, make a corresponding endpoint.
        Attr(ref attr) => {
            Rc::new(DeviceNodeEndpoint(DeviceEndpoint{
                attribute: root.clone(),
                // get the default value from the attribute to initialize
                value: Cell::new(attr.default)
            }))
        }
    }
}

// at the moment this only understands how to patch one contiguous section of a dmx universe
pub fn patch<'p>(profile: &'p Profile, device_tree_root: &mut DeviceBranch, addr: uint, univ: Rc<RefCell<DmxUniverse>> ) -> Option<Device<'p>> {
    match profile.chan_alloc {
        DmxSingleton(len) => {

            // build the corresponding DeviceNode for the root ProfileNode and put it in the tree
            device_tree_root.children.push( device_subtree_from_profile_subtree(&'p profile.root) );

            let d = Device {
                profile: profile,
                name: profile.name.clone(),
                nickname: profile.nickname.clone(),
                id: 0,
                patches: vec!(DevicePatch::new_dmx(addr, len, univ)),
                // unwrap() is safe here as we just pushed an element onto the vector so it cannot be empty.
                root: device_tree_root.children.last().unwrap().clone()
            };

            Some(d)
        },
        //_ => None
    }

}
