//! Models describing device profiles and concrete devices (as trees).
use dmx::DmxAddr;
use dmx::DmxMap;
use effect::EffectType;
use effect::EffectSubtype;
use effect::EffectSubsubtype;
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

// TODO - optional custom labels for each node? currently just default to profile node labels
struct DeviceBranch {
    profile_branch: Box<ProfileBranch>,
    children: ~[Device]
}

struct DeviceEndpoint {
    attribute: Box<Attribute>,
    value: Option<AttributeValue>, // required if rendering is implemented for this attribute
}
