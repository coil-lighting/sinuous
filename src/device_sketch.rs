// Nude pencil drawings of basic libsinuous constructs.
//
// This is just a napkin sketch for the port of rbld50 core from Ruby to Rust.
// There is a LOT more to come. Fortunately the Ruby stuff mostly tests and runs
// okay.
//
// This is not a well-organized, well-documented or well thought out module.
// It is only my very first Ruby program since FuzzBuzz and Hello, World.
// Please don't take it too seriously.
//
// It compiled under 0.9. Doesn't yet under 0.10-pre, thanks to changes to
// hashmap.

// # Comments that look like this are just copied verbatim from the Ruby version.
// # They do not necessarily pertain to this Rust version. Just for reference
// # while I try to port this.

extern crate collections;

//use std::f64;
//use std::num::Primitive;
//use std::u64;
use collections::HashMap;

// using enums as unions for now, hopefully this is ok. TODO: check memory layout


// Topology descriptors will describe the parametric range for an attribute's
// value.
// TODO determine whether we could use the topo types themselves as values in
// the topology array.
// TODO formally impose ranges using constraints from the type system?
// (Is this even possible in Rust?)
// XXX "Vector" means "array" in Rust, but here it means "continuous"
// TODO heterogeneous topologies in >1D attrs?
enum Topo {
    // Naturally continuous, values bounded, interpolation recommended.
    // Range: [0.0,1.0]
    // Example: dimmer
    VectorEuclidianUnipolar=0,

    // Naturally continuous, values bounded, interpolation recommended.
    // Range: [-1.0,1.0]
    // Example: X- or Y-position on a bounded pivot or linear track
    VectorEuclidianBipolar,

    // Naturally continuous, values wrap, interpolation recommended.
    // Range: [0.0,1.0]
    // Example: angle of rotation
    VectorToroidalUnipolar,

    // Naturally continuous, values wrap, interpolation recommended, with a
    // natural center point at 0.
    // Range: [-1.0,1.0]
    // Example: fully commutated pan or tilt
    VectorToroidalBipolar,

    // Naturally discontinuous, values wrap, interpolation conceivably
    // mechanically/logically meaningful, but aesthetically discouraged.
    // Range: Int indexed from 0
    // Example: litho index
    DiscreteRing,

    // Naturally discontinuous, values bounded, interpolation conceivably
    // mechanically/logically meaningful, but aesthetically discouraged.
    // Range: Int indexed from 0
    // Example: linear 35mm slide tray index
    DiscreteArray,

    // Naturally discontinuous, values bounded, interpolation
    // mechanically/logically inconceivable and therefore forbidden.
    // Range: Int indexed from 0
    // Example: color wheel mode
    DiscreteSet,

    // Topology is undefined, probably because this is a (virtual?) "cluster"
    // parent node. TODO: expand on this, accounting for the new device model
    // Range: null
    // Example: TODO
    UndefinedTopo,
}

// Named subtypes for the primitive storage representing the numeric value for
// a Device Attribute's instance.
// TODO: constrain according to notes in topo
// TODO: determine wheter we can actually imply topo through default values' types
enum AttributeValue {
    VectorEuclidianUnipolarValue(f64),
    VectorEuclidianBipolarValue(f64),
    VectorToroidalUnipolarValue(f64),
    VectorToroidalBipolarValue(f64),
    DiscreteRingValue(u64),
    DiscreteArrayValue(u64),
    DiscreteSetValue(u64),

    // Will this really work for cluster parents? Virtual only?
    // Does this mean that nonvirtual [...notes trail off here...] XXX
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
    DmxAddressOffsetSingle(u32), // TODO constrain to positive u32? really positive u9.
    DmxAddressOffsetMultiple(~[u32]),
    DmxAddressOffsetMap(HashMap<~str, u32>),
}

// # dmx_range -- The range for this attribute's value. May be multidimensional.
// # Informs the low-level rendering function how to interpret this
// # Attribute's value in terms of the output protocol.
// #
// # If this is a continuous polar attr, it should probably be a tuple like
// #   ((0,1), (2,125), (126,128))
// #   ...that is...
// #   ((min_min, min_max), (min+1_min, max-1_max), (max_min, max_max))
// #
// # Bipolar attribute probably has this structure
// #   ((negative_min,negative_max),(neutral_min,neutral_max),(positive_min,positive_max))
// #
// # Min and max can be ascending, equal,or descending like so
// #   ((255, 255), (254, 130), (129, 129))
// #
// # If this is a boolean, it should be
// #   ((false_min,false_max),(true_min,true_max))
// # ...where _min is always <= _max
// #
// # TODO: If this Attribute is an int or index type, shouldn't it take a
// # range matrix of ((val0_min,val0_max),(val1_min,val1_max)) and so on?

struct Range <T> {
    min: T,
    max: T,
}

type DmxValueRange = Range<u8>;

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
      ColorspaceRgb       =  1,
      ColorspaceHsb       =  2,
      ColorspaceRgbi      =  3,
      ColorspaceRgbw      =  4,
      ColorspaceRgbaw     =  5,
      ColorspaceHsl       =  6,
      Colorspace1x        =  7,
      Colorspace2x        =  8,
      Colorspace3x        =  9,
      Colorspace4x        = 10,
      Colorspace5x        = 11,
      ColorspaceI1x       = 12,
      ColorspaceI2x       = 13,
      ColorspaceI3x       = 14,
      ColorspaceI4x       = 15,
      ColorspaceI5x       = 16,
      Colorspace_i        = 17,
      Transform_rotate    = 18,
      Transform_scroll    = 19,
      Transform_translate = 20,
      Filter_misc         = 21, // e.g. an effects wheel where each slot does something different
      Filter_multiply     = 22, // e.g. prism, pyramid mirror, barrel mirror, moonflower mirror, disco ball
      Filter_distort      = 23, // e.g. textured glass
      Filter_subtract     = 24, // e.g. vignette, gobo
      Filter_add          = 25, // e.g. laser on technobeam
      Orientation_mirror  = 26,
      Orientation_yoke    = 27,
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
///     ranges: ~[DmxValueRange], // e.g. pack pan into value 127...256
/// }

struct DmxMap {
    address: u8, // profile not universe
    offset: DmxAddressOffset, // e.g. pan is channel 3
    ranges: DmxValueRange, // e.g. pack pan into value 127...256
}

struct Attribute {
    name: ~str, // e.g. "iris"

    /// was topology: ~[Topo],
    topology: ~Topo,

    /// Experimentally eliminating dimensionality in order to simplify device
    /// modeling. A (sub)Profile/sub(Device)'s dimensionality is just the
    /// number of its children. This allows us to break down an xy cartesian
    /// coordinate into separate x and y effects; we'll just have to decide
    /// how to indicate that we want to blend in higher dimensions, i.e. blend
    /// the xy branch, not x and y separately, for blend modes where x and y
    /// are independent. This might be tricky, but it's no trickier than trying
    /// to work around the atomicity of an xy 2D attribute for cases where I
    /// really just want to deal with x.

    /// Specify the number of dimensions in this Attribute.
    /// A 2D attribute (x,y) has dimensionality=2.
    /// A 1D attribute (dimmer level) has dimensionality=1.
    /// dimensionality: u32,

    effect: (EffectType, EffectSubtype, EffectSubsubtype),

    // TODO is 'virtual' needed anymore? might be able to make it virtual by enabling/disabling peer subdevices
    // virtual: bool,

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
    root: ~ProfileNode,
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
    address: u32, // TODO: constrain
}

enum Addr {
    DmxAddrType(DmxAddr), // TODO universe + address
    // MidiAddrType,
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
// 1) A 'physical' device, or rather a specific instance, i.e. one animated model
// in an external editor or one fixture on a tree.
// 2) An 'output' device (for lack of a better name), which is an addressed
// device on a specific universe. Whether we want to allow multiple output ports
// per universe is a separate question. It would allow us to make a soft DMX
// splitter without complicating the first-order output mappings associated with
// devices in the scenegraph. This might be called a 'logical device' or something.
// Or a 'patched device.' (Perhaps a universe gets patched, too.)
//     - these are managed by DevicePatches.
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
    root: ~DeviceNode,
}

// TODO - optional custom labels for each node? currently just default to profile node labels
struct DeviceBranch {
    profile_branch: ~ProfileBranch,
    children: ~[Device]
}

struct DeviceEndpoint {
    attribute: ~Attribute,
    value: AttributeValue,
}

// -----------------------------------------------------------------------------
// first stab at porting AttributeBlenderMethods.rb

// -----------------------------------------------------------------------------
// Low-level wrapping, normalization, and limiting functions

// TODO: is it possible to have a generic decl for sort_double_*64 without using
// pointers?

// reorder (a, b) to (b, a) if b > a
fn sort_double_i64(a: i64, b: i64) -> (i64, i64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

// reorder (a, b) to (b, a) if b > a
fn sort_double_f64(a: f64, b: f64) -> (f64, f64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

// call f on a and b before returning the sorted results
fn sort_apply_f64(f: fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    sort_double_f64(f(a), f(b))
}

// Given a toroidal Float value, wrap it around if it is out of the range 0...1.0 (incl.).
fn fVecTorusUniWrap(n: f64) -> f64 {
    // do not *always* fmod, or 1.0 is out of range
    if n > 1.0 {
        // TODO: verify Rust's % works like Ruby % modulo
        // https://github.com/mozilla/rust/issues/4565
        // https://github.com/mozilla/rust/pull/5990
        // In Python this had to be "1.0 + fmod(n,1.0)"
        if n % 1.0 == 0.0 {
            // Map 2.0 => 1.0, not => 0.0, so we can hit 1.0
            1.0
        } else {
            n % 1.0
        }
    } else if n < 0.0 {
        if n % 1.0 == 1.0 {
            0.0
        } else {
            n % 1.0
        }
    } else {
        n
    }
}

// Given a toroidal Float value, wrap it around if it is out of the range -1.0...1.0 (incl.).
fn fVecTorusBiWrap(n: f64) -> f64 {
    // do not *always* fmod, or 1.0 is out of range
    if n > 1.0 {
        let m = (n + 1.0) % 2.0;
        if m == 0.0 {
            // map 3.0 => 1.0 rather than -1.0
            1.0
        } else {
            m - 1.0
        }
    } else if n < -1.0 {
        // in python, this was 1.0 + fmod(n + 1.0,2.0)
        ((n+1.0) % 2.0) - 1.0
    } else {
        n
    }
}

fn fLimitUniUnit(n: f64) -> f64 {
    if n >= 1.0 {
        1.0
    } else if n <= 0.0 {
        0.0
    } else {
        n
    }
}

fn fLimitBiUnit(n: f64) -> f64 {
    if n >= 1.0 {
        1.0
    } else if n <= -1.0 {
        -1.0
    } else {
        n
    }
}

fn iVecEuclidLimit(n: i64, minimum: i64, maximum: i64) -> i64 {
    if n >= maximum {
        maximum
    } else if n <= minimum {
        minimum
    } else {
        n
    }
}

fn iVecTorusWrap(n: i64, minimum: i64, maximum: i64) -> i64 {
    if n > maximum {
        let d = 1 + maximum - minimum;
        let mut nn = n;
        while nn > maximum {
            // FIXME: convert to i64-friendly multiplication after a test is written
            nn -= d;
        }
        nn
    } else if n < minimum {
        let d = 1 + maximum - minimum;
        let mut nn = n;
        while nn > maximum {
            // FIXME: see above
            nn += d;
        }
        nn
    } else {
        n
    }
}

fn iVecTorusMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    // Clamp the input values, for sanity (TEMP?)
    let (aa, bb) = sort_double_i64(iVecTorusWrap(a, minimum, maximum),
                                   iVecTorusWrap(b, minimum, maximum));

    let rng = maximum - minimum + 1;
    let d = aa - bb;

    let highRoad: f64 = (rng - d) as f64 / 2.0;
    let lowRoad: f64 = d as f64 / 2.0;

    // choose the shortest route
    if highRoad < lowRoad {
        // wrap it around
        let n: i64 = (aa as f64 + highRoad).round() as i64;
        if n > maximum {
            n - rng
        } else {
            n
        }
    } else {
        (aa as f64 - lowRoad).round() as i64
    }
    // TODO unit test for this to verify rounding
}

fn iblendVecEuclidAdd(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    iVecEuclidLimit(a + b, minimum, maximum)
}

fn iblendVecEuclidSubtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    iVecEuclidLimit(a - b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
fn iblendVecTorusAdd(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    iVecTorusWrap(a + b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
fn iblendVecTorusSubtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    iVecTorusWrap(a - b, minimum, maximum)
}

// Return the average of a and b, rounding to the nearest integer.
// Rounding of *.5 numbers (1.5, -1.5) follows the behavior of Rust's
// round function, which rounds positive halves up and negative halves
// down.
fn iblendVecEuclidMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    iVecEuclidLimit(((a + b) as f64 / 2.0).round() as i64, minimum, maximum)
}

// TODO skipped: iblendVecEuclidMultiply (though it could be done).
// (Let's wait until we can practically experiment.)

fn iblendVecTorusMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    iVecTorusMedian(a, b, minimum, maximum)
}

// Given a Euclidian value a and b, return a new value which is the maximum of
// the two.
fn iblendVecEuclidMax(a: i64, b: i64) -> i64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
fn iblendVecEuclidMin(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
fn iblendVecEuclidAbsMax(a: i64, b: i64) -> i64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of iblendVecEuclidAbsMax. If a and b have the same absolute value,
// return b.
fn iblendVecEuclidAbsMin(a: i64, b: i64) -> i64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

// -----------------------------------------------------------------------------
// Generic blenders

// Given two attribute values a and b, just return a copy of a, "clobbering" b.
fn blendClobber<T>(a: T, _b: T) -> T {
    // XXX test that return value for >1D inputs are *copies*
    // this used to take dims because the ruby impl switched on dims; might not
    // be needed in rust
    a
}

// IDEA: do away with dimensionality. make compound attributes just subtrees
// of device space. then we can always unbundle, say, an xy effect into an x
// and a y without having to write a separate model branch in the profile.
// also, it gives us the option to have heterogeneous topos and datatypes
// within a compound attribute.

// IDEA: make an enum that is either a num or a List<num>,
// write a generic list dupper if somehow it doesn't already exist,
// don't even take dims -- its purpose seems to be to differentiate between
// 1Ds, which show up as primitives, and >1Ds, which show us as sequences



// -----------------------------------------------------------------------------
// Float blenders

// TODO: some of these might be made generic on primitives like this:
// pub fn abs<T: Signed>(value: T) -> T
// If we can do this without pointers, do so in a separate clean-up step.

// Given a Euclidian value a and b, return a new value which is the maximum of
// the two.
fn fblendVecEuclidMax(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
fn fblendVecEuclidMin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
fn fblendVecEuclidAbsMax(a: f64, b: f64) -> f64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of fblendVecEuclidAbsMax. If a and b have the same absolute value,
// return b.
fn fblendVecEuclidAbsMin(a: f64, b: f64) -> f64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

fn fblendVecEuclidBiAdd(a: f64, b: f64) -> f64 {
    fLimitBiUnit(a + b)
}

fn fblendVecEuclidBiSubtract(a: f64, b: f64) -> f64 {
    fLimitBiUnit(a - b)
}

fn fblendVecEuclidUniAdd(a: f64, b: f64) -> f64 {
    fLimitUniUnit(a + b)
}

fn fblendVecEuclidUniSubtract(a: f64, b: f64) -> f64 {
    fLimitUniUnit(a - b)
}

fn fblendVecTorusUniAdd(a: f64, b: f64) -> f64 {
    fVecTorusUniWrap(a + b)
}

fn fblendVecTorusUniSubtract(a: f64, b: f64) -> f64 {
    fVecTorusUniWrap(a - b)
}

// If a + b is out of range, wrap it. centers around 0. Could be really weird.
fn fblendVecTorusBiAdd(a: f64, b: f64) -> f64 {
    fVecTorusBiWrap(a + b)
}

// If a - b is out of range, wrap it. centers around 0. Could be really weird.
fn fblendVecTorusBiSubtract(a: f64, b: f64) -> f64 {
    fVecTorusBiWrap(a - b)
}

fn fblendVecEuclidMedian(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    (a + b) / 2.0
}

fn fblendVecEuclidMultiply(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    a * b
}

// private impl for 2 fns below. TODO: rename
fn _fblendVecTorusMedian(f: fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let (aa, bb) = sort_apply_f64(f, a, b);
    let highRoad = 1.0 - aa + bb;
    let lowRoad = aa - bb;

    // choose the shortest route
    if highRoad < lowRoad {
        f(aa + highRoad/2.0)
    } else {
        (aa + bb) / 2.0
    }
}

fn fblendVecTorusUniMedian(a: f64, b: f64) -> f64 {
    _fblendVecTorusMedian(fVecTorusUniWrap, a, b)
}

fn fblendVecTorusBiMedian(a: f64, b: f64) -> f64 {
    _fblendVecTorusMedian(fVecTorusBiWrap, a, b)
}

// dimensions of variation:
//    blenders:
//      datatype (per dim?)
//      vec vs. primitive (reduce to all vec?) promote to -toplevel?
//      topo } is topo+op really one dim?
//      op   }
//      I think datatype+topo is really one thing, because topology is about
//      what is adjacent to something, which obviously depends on datatype

fn main() {
    println!("Done.")
}
