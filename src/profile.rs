//! Models for describing device types (profiles) as digraphs.

use std::cell::RefCell;
use std::rc::Rc;

use dmx::DmxMap;
use effect::EffectType;
use effect::EffectSubtype;
use effect::EffectSubsubtype;
use topo::Topo;

/// Hypothesis: devices' descriptions are trees of ProfileElements, and this will
/// suffice to describe everything from simple, 1 dimensional, nonmodal
/// attributes like a dimmer channel to complex, multidimensional, modal
/// attribute clusters such as mspeed-smoothed continuous litho wheel angle.
pub struct Profile {
    pub name: String,         // "Technobeam"
    pub nickname: String,     // "Techno"
    pub manufacturer: String, // "HES"
    pub author: String,       // "e.g. Steve Jobs"
    pub date: String,       // maybe we want to make this some kind of timestamp type
    pub version: int,       // 1, 2, 3...
    pub chan_alloc: ChannelAlloc, // what kinds of addresses do we need to allocate to patch one?

    pub root: Rc<RefCell<ProfileGraph>>,
}

/// We will gradually expand the ways we can allocate channels, potentially
/// across multiple universes and even protocols.
pub enum ChannelAlloc {
    DmxChannelCount(uint),
}

/// A node in a graph of profile nodes. This node might represent a single
/// device type, or a single data field in a device descriptor, or a type of
/// assembly of multiple devices, or a discrete subsystem in a type of device.
///
/// Note that although aggregations of profile nodes are acyclic digraphs,
/// aggregations of device nodes are simply trees.
///
/// Once instantiated, a ProfileGraph node's state is immutable. Nodes may
/// therefore safely share parents. This makes it possible to efficiently
/// encode small variations between similar profiles. Similar profiles often
/// arise across product ranges, e.g. Technobeam, Technobeam-i, and Mac 250.
/// Similarities also arise across lineages of customization; for example, I
/// might have customized the color wheel in some of my Technobeams, and the
/// effect wheel in others, but they all derive from the same OEM profile.
/// Because we model profiles as append-only acyclic digraphs, the cost of
/// encoding a variant is just the cost of the differences.
pub enum ProfileGraph {
    ProfileGraphAttribute(Attribute), // a leaf node
    ProfileGraphBranch(ProfileBranch), // a normal branch node (like a folder)
    ProfileGraphSwitch(ProfileSwitch), // an exclusive modal switch branch node
}

// Named subtypes for the primitive storage representing the numeric value for
// a Device Attribute's instance.
#[deriving(Clone)]
pub enum AttributeValue {
    Continuous(f64),
    Discrete(i64), // TODO - decide whether to make this unsigned instead
}

/// A leaf node: usually, but not always, a renderable datapoint.
pub struct Attribute {
    pub name: String, // e.g. "Gobo wheel 1 position"
    pub nickname: String, // e.g. "Gobo1"
    pub effect: (EffectType, EffectSubtype, EffectSubsubtype),
    pub topo: &'static Topo,
    pub default: Option<AttributeValue>, // required if rendering is implemented
    pub dmx: Option<DmxMap>, // required if DMX rendering is implemented
}

/// An ordinary, inclusive branch node, used to group Profile subgraphs.
///
/// For example, all of the leaf Attributes of a simple device type might be
/// peers, sharing a common parent ProfileBranch. You might use this pattern to
/// describe a six channel dimmer pack, where the dimmer pack is a ProfileBranch
/// node with six child Attributes, one per channel.
pub struct ProfileBranch {
    pub name: String, // "Technobeam"
    pub nickname: String, // "Techno"
    pub children: Vec<Rc<RefCell<ProfileGraph>>>,
}

/// A switching modal branch node, which declares that only one of its
/// children is renderable. For example, you might use this to switch on and
/// off onboard temporal interpolation for a device. (Sometimes a switch is
/// just a boolean Attribute endpoint, but certain manufacturers, especially
/// in the early days of moving lights, responded to the limited channel
/// range of DMX and available control boards by packing multiple attributes
/// and multiple device modalities into very few channels. This made life
/// difficult for profile library developers and arguably hobbled software
/// evolution across the lighting industry. ProfileSwitch demonstrates
/// an easy way to split a large number of mutally exclusive behaviors into
/// multiple branches of a profile, even when those numerous behaviors are
/// eventually serialized as a smaller number of channels.
///
/// TODO: Deserialization algorithm. (May require more data fields here.)
pub struct ProfileSwitch {
    pub name: String, // "Motor smoothing"
    pub nickname: String, // "MSpeed"
    pub children: Vec<Rc<RefCell<ProfileGraph>>>,
    pub default_selection: uint, // TODO: might need to be Option<uint> for construction purposes, but hopefully not
}
