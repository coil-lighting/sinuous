//! Models describing device concrete devices (as trees).

// I experimentally eliminated dimensionality in order to simplify device
// modeling. A (sub)Profile/sub(Device)'s dimensionality is just the
// number of its children. This allows us to break down an xy cartesian
// coordinate into separate x and y effects; we'll just have to decide
// how to indicate that we want to blend in higher dimensions, i.e. blend
// the xy branch, not x and y separately, for blend modes where x and y
// are independent. This might be tricky, but it's no trickier than trying
// to work around the atomicity of an xy 2D attribute for cases where I
// really just want to deal with x. - MB

use std::cell::RefCell;
use std::cell::Cell;
use std::rc::Rc;

use dmx::*;
use profile::*;
use render::*;
use world::Loc;

/// A device is an actual instance of a device. A device is described by its
/// Profile tree.
///
/// Note that although aggregations of profile nodes are acyclic digraphs,
/// aggregations of device nodes are simply trees.
///
/// Unlike their profiles, devices are trees because their payload is mutable,
/// so a device node must never be incorporated by reference into multiple
/// parent devices.
pub enum DeviceTree {
    DeviceTreeEndpoint(DeviceEndpoint),
    DeviceTreeBranch(DeviceBranch),
    DeviceTreeSwitch(DeviceSwitch),
}

pub enum Addr {
    DmxAddrType(DmxAddr), // TODO universe + address?
    // Midi_addrType,
    // OscAddrType,
    // OpenPixelControlAddrType,
    // ...
}

pub struct DeviceEndpoint {
    /// This must be specially a reference to an Attribute, not any old
    /// ProfileGraph, but we haven't figured out how to express the specialized
    /// reference.
    pub attribute: Rc<RefCell<ProfileGraph>>,

    /// Required only if rendering is implemented for this attribute.
    pub value: Cell<Option<AttributeValue>>,
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

    /// Fail if you have corrupted the device tree by linking an endpoint to
    /// a non-Attribute ProfileBranch.
    pub fn render(&self, buffer: &mut[u8]) {

        let at_ref = self.attribute.borrow();

        let attribute = match *at_ref {
            ProfileGraphAttribute(ref a) => a,
            _ => fail!("Every DeviceEndpoint must be bound to a ProfileGraphAttribute, not a ProfileGraphBranch."),
        };

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
                // TODO: fix render to have explicit lifetime specifier to avoid the copy here:
                r(ni, range.clone(), offset, buffer);
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

/// A branch node in a tree of device nodes. This node might represent a single
/// instrument, or a group of instruments, or a subsystem in a single
/// instrument.
pub struct DeviceBranch {
    /// Only concrete devices need profiles. Abstract devices don't -- for
    /// example, a whole show could be a branch device, which you might split
    /// into many rooms, each of which is likewise its own branch device,
    /// potentially containing several assemblies (two different trusses, for
    /// instance). Profiles are only strictly necessary for rendering subtrees
    /// of the device tree.
    pub profile_branch: Option<Rc<RefCell<ProfileGraph>>>,
    pub children: Vec<Rc<RefCell<DeviceTree>>>,
}

impl DeviceBranch {
    pub fn render(&self, buffer: &mut[u8]) {
        for child in self.children.iter() {
            match *child.borrow() {
                // Rust manual: "Patterns that bind variables default to binding
                // to a copy or move of the matched value (depending on the
                // matched value's type). This can be changed to bind to a
                // reference by using the 'ref' keyword, or to a mutable
                // reference using 'ref mut'."
                DeviceTreeEndpoint(ref d) => d.render(buffer),
                DeviceTreeBranch(ref d) => d.render(buffer),
                DeviceTreeSwitch(ref d) => d.render(buffer),
            };
        }
    }
}

/// A switching modal branch node in a tree of device nodes. This node typically
/// represents a modal feature in a single instrument, or (very rarely) a mode
/// of a whole group of instruments, or a complex modal subsystem in a single
/// instrument.
pub struct DeviceSwitch {
    /// Only concrete devices need profiles. Abstract devices don't -- for
    /// example, a whole show could be a branch device, which you might split
    /// into many rooms, each of which is likewise its own branch device,
    /// potentially containing several assemblies (two different trusses, for
    /// instance). Profiles are only strictly necessary for rendering subtrees
    /// of the device tree.
    pub profile_branch: Option<Rc<RefCell<ProfileGraph>>>,
    pub children: Vec<Rc<RefCell<DeviceTree>>>,

    /// The array index of the selected child branch
    pub selection: uint,
}

impl DeviceSwitch {
    pub fn render(&self, buffer: &mut[u8]) {
        let child = self.children.get(self.selection);
        match *child.borrow() {
            DeviceTreeEndpoint(ref d) => d.render(buffer),
            DeviceTreeBranch(ref d) => d.render(buffer),
            DeviceTreeSwitch(ref d) => d.render(buffer),
        };
    }
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
    pub root: Rc<RefCell<DeviceTree>>,
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
        //
        // TODO do not render redundantly if patched more than once (with the same protocol)?
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

                            match *self.root.borrow() {
                                DeviceTreeEndpoint(ref d) => d.render(buffer),
                                DeviceTreeBranch(ref d) => d.render(buffer),
                                DeviceTreeSwitch(ref d) => d.render(buffer),
                            };


                        },
                        None => () // if something else is already writing to
                        // the universe buffer, we can't get access.  give up.
                    }
                }
            }
        }
    }
}

pub fn device_subtree_from_profile_subtree(root: &Rc<RefCell<ProfileGraph>>) -> Rc<RefCell<DeviceTree>> {
    match *root.borrow() { // Rc implements the trait deref, so the * operator works.
        // If this is a leaf, make a corresponding endpoint.
        ProfileGraphAttribute(ref attr) => {
            Rc::new(RefCell::new(DeviceTreeEndpoint(DeviceEndpoint{
                attribute: root.clone(),
                // get the default value from the attribute to initialize
                value: Cell::new(attr.default),
            })))
        },
        // If this is a profile branch, recursively construct its subtree.
        // Recursively collect all children into a vector.
        ProfileGraphBranch(ref pb) => {
            Rc::new(RefCell::new(DeviceTreeBranch(DeviceBranch {
                profile_branch: Some(root.clone()),
                children: pb.children.iter().map(|pb_child|
                	device_subtree_from_profile_subtree(pb_child)).collect(),
            })))
        },
        // TODO deduplicate with respect to ProfileGraphBranch. (match syntax?)
        // If this is a profile switch, recursively construct its subtree.
        // Recursively collect all children into a vector.
        ProfileGraphSwitch(ref pb) => {
            Rc::new(RefCell::new(DeviceTreeSwitch(DeviceSwitch {
                profile_branch: Some(root.clone()),
                children: pb.children.iter().map(|pb_child|
                	device_subtree_from_profile_subtree(pb_child)).collect(),
                selection: pb.default_selection,
            })))
        },
    }
}

// at the moment this only understands how to patch one contiguous section of a dmx universe
pub fn patch<'p>(profile: &'p Profile, device_tree_root: Rc<RefCell<DeviceBranch>>, addr: uint, univ: Rc<RefCell<DmxUniverse>> ) -> Option<Device<'p>> {
    match profile.chan_alloc {
        DmxChannelCount(len) => {

            // build the corresponding DeviceTree for the root ProfileGraph and put it in the tree
            device_tree_root.borrow_mut().children.push( device_subtree_from_profile_subtree(&profile.root) );

            let d = Device {
                profile: profile,
                name: profile.name.clone(),
                nickname: profile.nickname.clone(),
                id: 0,
                patches: vec!(DevicePatch::new_dmx(addr, len, univ)),
                // unwrap() is safe here as we just pushed an element onto the vector so it cannot be empty.
                root: device_tree_root.borrow().children.last().unwrap().clone()
            };

            Some(d)
        },
        //_ => None
    }

}
