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

extern crate collections;

use collections::HashMap;
use range::DmxRange;
use topo::Topo;

mod numeric;
mod range;
mod render;
mod blend;
mod topo;

// Named subtypes for the primitive storage representing the numeric value for
// a Device Attribute's instance.
// TODO: constrain according to notes in topo
// TODO: determine wheter we can actually imply topo through default values' types
enum AttributeValue {
    ContinuousEuclidianUnipolarValue(f64),
    ContinuousEuclidianBipolarValue(f64),
    ContinuousRingUnipolarValue(f64),
    ContinuousRingBipolarValue(f64),

    // TODO decide whether discrete attributes should be signed or unsigned
    // (and propagate that decision to blend.rs)
    DiscreteRingValue(i64),
    DiscreteArrayValue(i64),
    DiscreteSetValue(i64),

    // For device nodes with no associated value (maybe don't need this)
    UndefinedValue(()),
}

// TODO figure out how Rust wants us to associate functions directly with types.
// No need to pay the price for a hash lookup on static constant associations.
enum AttributeType {
    // name                     *dmx* renderer (old Ruby style map)
    ModalParent          ,// => LrenderDMXModalParent,
    IndexVirtual         ,// => LrenderDMXVirtual,
    Cluster              ,// => LrenderDMXCluster,
    IndexWithRange       ,// => LrenderDMXIntIndexedWithRange,
    BooleanWithRange     ,// => LrenderDMXBooleanWithRange,
    Float                ,// => LrenderDMXFloat,
    FloatBipolar         ,// => LrenderDMXFloatBipolar,
    FloatWithRange       ,// => LrenderDMXFloatWithRange,
    FloatBipolarWithRange,// => LrenderDMXFloatBipolarWithRange,
    Double               ,// => LrenderDMXDouble,
    FloatArray           ,// => LrenderDMXFloatArray,
    DoubleArray          ,// => LrenderDMXDoubleArrayBigEndian,
    DoubleArrayInterlaced,// => LrenderDMXDoubleArrayBigEndianInterlaced,
    DoubleArrayBipolar   ,// => LrenderDMXDoubleArrayBipolarBigEndian,
    FloatArrayMapped     ,// => LrenderDMXFloatArrayMapped,
    SpinBipolar2Ch       ,// LrenderDMXSpinBipolar2ChWithRange,
    //TODO: :doubleArrayBipolarInterlaced => :renderDMXDoubleArrayBipolarBigEndianInterlaced,
}

// # dmx_offset - Specifies insertion order within the serialized output.
// # For DMX, this specifies byte offset (AKA channel offset) within the
// # fixture's data array. Offset may be an int or a sequence of ints.
// # If dmx_offset is an int, then this Attribute's rendered data is packed
// # sequentially starting at the given offset. If dmx_offset is an int
// # sequence, then this Attribute's rendered data occupies multiple channels
// # which are not necessarily contiguous. Each rendered datum is rendered at
// # its corresponding offset. For example, datum[2] would be rendered at
// # offset[2]. This typically happens in a CMYK profile where K is not
// # adjacent to C+M+Y. Only certain attribute types require mapped-
// # multichannel (sequential) dmx_offsets; see also
// # DMXAttributeRenderers.attributeTypes.
// #
// # It is the responsibility of self.renderDMX() to interpret dmx_offset.
enum DmxAddressOffset {
    //  a map or an array or an int... anything else?
    DmxAddressOffsetSingle(uint),
    DmxAddressOffsetMultiple(~[uint]),
    DmxAddressOffsetMap(HashMap<~str, uint>), // TODO Is it really necessary to use a hashmap here? I sure hope not.
}

// Matrix-mappable effect ((sub)sub)type (hint) metadata (EXPERIMENTAL).
// Conceptually, types, subtypes, and subsubtypes exist in a 3D space of
// options. Currently there are about 1000 option combinations total (see
// FECT_MAXIMA). Many combinations don't actually make sense from a design
// standpoint.
//
// From Ruby (probably no longer needed):
// the maximum size of a 3D array required to map all effects combos
//     FECT_MAXIMA = [17,27,2].freeze
//
// These type hints must not impact DMX rendering! They may impact the
// display of previews, icons, or controls in your onscreen controller.
//
// In ruby, I put these types in order so they could be used as array indices,
// and to stabilize their display order. Rust might give us a better way to
// do this.
//
// FIXME these probably should be structs, not constants. they were constants
// because of the way they were wired up in Python.
enum EffectType {
      Misc              =   0,
      ModeSelect        =   1,
      Color             =   2,
      Dimmer            =   3,
      Relay             =   4,
      Focus             =   5,
      Zoom              =   6,
      Iris              =   7,
      Frame             =   8,
      Strobe            =   9,
      Position          =  10,
      Orientation       =  11,
      Transform         =  12,
      FilterSelect      =  13, // e.g. litho index
      FilterIntensity   =  14, // e.g. frost (should frost get its own filter type?)
      Raster            =  15,
      Control           =  16,
      Smoothing         =  17,
}

// Effect subtypes - where needed, dimensionality is separately specified.
//
// TODO Make sure there is a test that verifies that all these have impls.
//
// FIXME these probably should be structs, not constants. they were constants
// because of the way they were wired up in Python.
enum EffectSubtype {
      Other               =  0,
      ColorspaceRgb       =  1, // red + green + blue
      ColorspaceHsb       =  2, // hue + saturation + brightness
      ColorspaceRgbi      =  3, // i = intensity multiplied
      ColorspaceRgbw      =  4, // w = white added
      ColorspaceRgbaw     =  5, // a = amber added
      ColorspaceHsl       =  6,
      Colorspace1x        =  7, // one mono filter such as a dichro
      Colorspace2x        =  8, // two mono filters
      Colorspace3x        =  9, // etc.
      Colorspace4x        = 10,
      Colorspace5x        = 11,
      ColorspaceI1x       = 12, // intensity + one mono filter such as a dichro
      ColorspaceI2x       = 13, // intensity + two mono filters
      ColorspaceI3x       = 14, // etc.
      ColorspaceI4x       = 15,
      ColorspaceI5x       = 16,
      ColorspaceI         = 17, // just intensity, a.k.a. grayscale or dimmer
      TransformRotate     = 18,
      TransformScroll     = 19,
      TransformTranslate  = 20,
      FilterMisc          = 21, // e.g. an effects wheel where each slot does something different
      FilterMultiply      = 22, // e.g. prism, pyramid mirror, barrel mirror, moonflower mirror, disco ball
      FilterDistort       = 23, // e.g. textured glass
      FilterSubtract      = 24, // e.g. vignette, gobo
      FilterAdd           = 25, // e.g. laser on technobeam
      OrientationMirror   = 26,
      OrientationYoke     = 27,
 }
// TODO add frame leaf order / orientation? NSEW, NESW... (N=North, S=South...)
// Or just impose a normal leaf order and allow offset map to scramble it?

enum EffectSubsubtype {
      Value     = 0, // e.g. intensity, must be 0
      Frequency = 1, // e.g. rotation speed
      Duration  = 2, // e.g. mspeed smoothing time
}

/// struct DmxMap {
///     addresses: ~[u8], // 1 or more, relative to profile not universe
///     offset: ~[DmxAddressOffset], // e.g. pan is channel 3
///     ranges: ~[DmxRange], // e.g. pack pan into value 127...256
/// }

struct DmxMap {
    address: uint, // profile not universe
    offset: DmxAddressOffset, // e.g. pan is channel 3
    ranges: DmxRange, // e.g. pack pan into value 127...256
}

struct Attribute {
    name: ~str, // e.g. "iris"
    topo: Box<Topo>,

    /// Experimentally eliminating dimensionality in order to simplify device
    /// modeling. A (sub)Profile/sub(Device)'s dimensionality is just the
    /// number of its children. This allows us to break down an xy cartesian
    /// coordinate into separate x and y effects; we'll just have to decide
    /// how to indicate that we want to blend in higher dimensions, i.e. blend
    /// the xy branch, not x and y separately, for blend modes where x and y
    /// are independent. This might be tricky, but it's no trickier than trying
    /// to work around the atomicity of an xy 2D attribute for cases where I
    /// really just want to deal with x.

    effect: (EffectType, EffectSubtype, EffectSubsubtype),

    attribute_type: AttributeType,
    dmx: DmxMap, // TODO: or null
    default: AttributeValue,

    // from rb, maybe no longer needed:
    // ONLY IF THIS ATTRIBUTE NEEDS LABELED INDICES
    //     @indexTable=nil // if required, a tuple of strings, one per mode, in order

    // ONLY IF THIS ATTRIBUTE GOVERNS A MODAL CLUSTER
    //     @clusterMethod=nil
}

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


// hopefully generalize this to be any output port, not just a dmx universe
struct DmxUniverse {
    id: u32, // TEMP
}

struct DmxAddr {
    universe: DmxUniverse,
    address: uint, // TODO: statically constrain if possible
}

enum Addr {
    DmxAddrType(DmxAddr), // TODO universe + address
    // Midi_addrType,
    // OscAddrType,
    // ...
}

struct Position {
    x: f64,
    y: f64,
    z: f64,
}

struct Orientation {
    pan: f64,  // yaw
    tilt: f64, // pitch
    roll: f64, // roll
}

struct Loc {
    position: Position,
    orientation: Orientation,
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
    value: AttributeValue,
}


fn main() {
    println!("Done.")
}
