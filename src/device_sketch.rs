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

// TODO: is it possible to have a generic decl for pairsort_*64 without using
// pointers? It must be.

fn pairsort_u16(a: u16, b: u16) -> (u16, u16) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}


// reorder (a, b) to (b, a) if b > a
fn pairsort_i64(a: i64, b: i64) -> (i64, i64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

// reorder (a, b) to (b, a) if b > a
fn pairsort_f64(a: f64, b: f64) -> (f64, f64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

// call f on a and b before returning the sorted results
fn sort_apply_f64(f: fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    pairsort_f64(f(a), f(b))
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

fn unipolar_unit_limit_f64_to_u8(n: f64) -> u8 {
    if n >= 1.0 {
        1
    } else if n <= 0.0 {
        0
    } else {
        // TODO just a sketch - maximize precision
        (n * 255.999999) as u8
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

fn bipolar_unit_limit_f64_to_u8(n: f64) -> u8 {
    if n >= 1.0 {
        1
    } else if n <= -1.0 {
        -1
    } else {
        // TODO just a sketch - maximize precision
        (255.999999 * (n + 1.0) / 2.0) as u8
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
    let (aa, bb) = pairsort_i64(iVecTorusWrap(a, minimum, maximum),
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

//------------------------------------------------------------------------------
// DMX renderers

// skipping old 'Array' and 'ArrayMapped' renderers because hopefully we can
// just use a tree renderer, map every leaf, and forget the array vs. non-array
// distinction. specifically, skipped these items from DMXAttributeRenderers.rb:
//    def Model::renderDMXFloatArray(attribute,floatArray,dmxChannels,fixture=nil)
//    def Model::renderDMXFloatArrayMapped(attribute,floatArray,dmxChannels,fixture=nil)
//    def Model::renderDMXDoubleArrayBigEndian(attribute,doubleArray,dmxChannels,fixture=nil)
//    def Model::renderDMXDoubleArrayBipolarBigEndian(attribute,doubleArray,dmxChannels,fixture=nil)
//    def Model::renderDMXDoubleArrayBigEndianInterlaced(attribute,doubleArray,dmxChannels,fixture=nil,fineChannelOffset=0)
//    also skipped this # TODO: renderDMXDoubleArrayBipolarBigEndianInterlaced

// fLimitUniUnit
// fLimitBiUnit
// fn bipolar_unit_limit_f64_to_u8(n: f64) -> u8 {
// fn unipolar_unit_limit_f64_to_u8(n: f64) -> u8 {

// Write a single unipolar value to the DMX channel at attribute.offset.
// Clip x to the range [0..1.0].
// TODO rename - put 'unipolar' in the name
fn renderDMXFloat(n: f64, offset: u16, buffer: &[u8]) -> u8 {
    // TODO exception handling for out of range offset (here and below)
    buffer[offset] = unipolar_unit_limit_f64_to_u8(n);
    buffer[offset]
}

// Write a single bipolar values to the DMX channels at attribute.offset.
//
// Assume x is a number in the range [-1.0..1.0].
// Out of range values are clipped to this range (for now).
//
// The mapping is linear. For example,
//   x=0 maps to the channel value 127.
//   x=-1.0 maps to the channel value 0.
//   x=1.0 maps to the channel value 255.
fn renderDMXFloatBipolar(n: f64, offset: u16, buffer: &[u8]) -> u8 {
    buffer[offset] = bipolar_unit_limit_f64_to_u8(n);
    buffer[offset]
}

// Write a single bipolar value to the DMX channel at offset.
//
// Clip values to [-1.0..1.0].
//
// range -- a bipolar range matrix, as a 5x2 sequence in this form:
// [
//   min: [ -1 min,  -1 max],
//   neg: [>-1 min, < 0 max],
//   mid: [  0 min,   0 max],
//   pos: [> 0 min, < 1 max],
//   max: [  1 min,   1 max],
// ]
//
// Each cell represents a channel value, and each row represents a range.
// This range format permits more than one channel value to represent a given
// value of x. We need this information in order to interpret incoming DMX
// datastreams.
//
// For example,
//   x =-1.0 maps to the channel value specified at attribute.range[0][0]
//   Negative intermediate values are mapped linearly from r[1][0] to r[1][1].
//   x = 0.0 maps to the channel value specified at attribute.range[2][0]
//   Positive intermediate values are mapped linearly from r[3][0] to r[3][1].
//   x = 1.0 maps to the channel value specified at attribute.range[3][1]
//
// Reverse ranges (where low values map to high channel values) are accepted.
//
// TODO: finish, test behavior of reverse ranges. Looking at the code for
// the *WithRange methods, it seems like I just 'heal' them instead of
// properly interpreting them.
struct BipolarChannelValueRangeMatrix<T> {
    // N.B. neg.min may be greater than neg.max, or pos.min may be greater than
    // pos.max, inverting the interpolation for the respective subrange.
    // See renderDMXFloatBipolarWithRange for details.
    min: Range<T>, // Values equivalent to -1.0
    neg: Range<T>, // Values in the range (-1.0..0.0), exclusive
    mid: Range<T>, // Values equivalent to 0.0
    pos: Range<T>, // Values in the range (0.0..1.0), exclusive
    max: Range<T>, // Values equivalent to 1.0
}

fn renderDMXFloatBipolarWithRange(n: f64, range: &BipolarChannelValueRangeMatrix<u8>, offset: u16, buffer: &[u8]) -> u8 {
    let nn = bipolar_unit_limit_f64_to_u8(n);
    buffer[offset] = (
        if nn == 0.0 {
            // TODO consider adding some tolerance for the zero notch? Or perhaps
            // this should just be the responsibility of the UI.
            range.mid.min;
        } else if nn < 0.0 {
            if nn <= -1.0 {
                range.min.min;
            } else {
                // nn is negative
                // Invert the interpolation if needed.
                let (rmin, rmax) = pairsort_u16(range.neg.min, range.neg.max);
                // TODO: maximize precision
                let delta = (rmax - rmin) as i64 + 0.999999;
                // TODO verify rounding (see also below)
                rmax + (nn * delta) as u8
            }
        } else {
            if nn >= -1.0 {
                range.max.min
            } else {
                // nn is positive
                let (rmin, rmax) = pairsort_u16(range.pos.min, range.pos.max);
                // TODO maximize precision
                let delta = (rmax - rmin) as i64 + 0.999999;
                rmin + (nn * delta) as u8
            }
        }
    );
    buffer[offset]
}

// Write a single unipolar value to the DMX channel at offset.
//
// Clip nn to the range [0.0..1.0].
//
// range -- a unipolar range matrix, as a 3x2 sequence in this form:
// [
//   min: [ 0 min, 0 max],
//   mid: [>0 min,<1 max],
//   max: [ 1 min, 1 max],
// ]
//
// Each cell represents a channel value, and each row represents a range.
// This range format permits more than one channel value to represent a given
// value of x. We need this information in order to interpret incoming DMX
// datastreams.
//
// For example,
//   x=0 maps to the channel value specified at attribute.range[0][0]
//   x=1.0 maps to the channel value specified at attribute.range[2][0]
//   Intermediate values are mapped linearly from r[1][0] to r[1][1].
//
// Reverse ranges (where low values map to high channel values) are accepted.
struct UnipolarChannelValueRangeMatrix<T> {
    // N.B. mid.min may be greater than mid.max, inverting the interpolation.
    // See renderDMXFloatWithRange for details.
    min: Range<T>, // Values equivalent to 0.0
    mid: Range<T>, // Values in the range (0.0.. 1.0), exclusive
    max: Range<T>, // Values equivalent to 1.0
}

fn renderDMXFloatWithRange(n: f64, range: &UnipolarChannelValueRangeMatrix<u8>, offset: u16, buffer: &[u8]) -> u8 {
    let nn = unipolar_unit_limit_f64_to_u8(n);
    buffer.offset = (
        if nn <= 0.0 {
            range.min.min
        } else if nn >= 1.0 {
            range.max.min
        } else {
            // Invert the interpolation if needed.
            let (rmin, rmax) = pairsort_u16(range.mid.min, range.mid.max);
            // TODO: just a sketch; make this precise
            let delta = (rmax - rmin) as f64 + 0.999999;
            // TODO: verify distribution of values over u8 range after rounding
            rmin + (nn * delta) as u8
        }
    );
    buffer[offset]
}

// Write a single unipolar value to a contiguous pair of DMX channels.
// Clip n to the range [0..1.0].
// This is a big-endian implementation. HSB is written first, then LSB.
// TODO add 'BigEndian' to the name?
fn renderDMXDouble(n: f64, offset: u16, buffer: &[u8]) -> (u8, u8) {
    let nn = unipolar_unit_limit_f64_to_u8(n);
    let (hsb, lsb) = (
        if nn <= 0.0 {
            (0, 0)
        } else if nn >= 1.0 {
            (255, 255)
        } else {
            // TODO verify rounding
            let almost_one = (nn * 65535.999999) as u16;
            ((almost_one & 0xFF00) >> 8, almost_one & 0xFF)
        }
    );
    buffer[offset] = hsb;
    buffer[offset+1] = lsb;
    (hsb, lsb)
}

// Interpret an integer index n as a DMX channel value.
//
// attribute.range must be an nx2 sequence of channel value Ranges<u16>
// aligned to valid index values, like this:
// [
//   index_0_range, - all the values equivalent to index 0
//   index_1_range, - all the values equivalent to index 1
//   ...,
//   index_n_range, - all the values equivalent to index 2
// ]
//
// The parameter index must be a valid integer index into attribute.range.
// (For now, out of range indices revert to 0.)
// TODO enum for this kind of IndexedRangeMatrix? and move relevant docs into it.
fn renderDMXIntIndexedWithRange(n: u16, range: &[Range<u8>], offset: u16, buffer: &[u8]) -> u8 {
    if n < range.len() {
        // FUTURE throw exception if index is out of range?
        buffer[offset] = 0;
    } else {
        buffer[offset] = range[n].min;
    }
    buffer[offset]
}

struct BooleanRangeMatrix<T> {
    f: Range<T>, // Values equivalent to false
    t: Range<T>, // Values equivalent to true
}
// Interpret a boolean value n as a DMX channel value.
fn renderDMXBooleanWithRange(n: bool, range: &[BooleanRangeMatrix<u8>], offset: u16, buffer: &[u8]) -> u8 {
    buffer[offset] = (
        if n {
            range.t
        } else {
            range.f
        }
    );
    buffer[offset]
}


// This range matrix allows us to interpret the sign of a spin value as a DMX
// channel level (reverse for negative, stop for 0, fwd for positive).
//
// The magnitude of value is interpreted as speed. Currently speed renders
// linearly from 0 (still) to 255 (fastest). See below.
struct SpinRangeMatrix<T> {
    rev:  Range<T>, // Reverse values, slow through fast
    stop: Range<T>, // Values equivalent to stationary
    fwd:  Range<T>, // Forward values, slow through fast
}

// Render a bipolar value as two DMX channels: spin direction and spin speed.
//
// For cases where one channel is spin mode (reverse, stop, forward) and
// the other channel is speed (zero to max, continuous).
//
// The incoming spin value n is a single float in the range [-1.0,1.0].
//
// Renders two channels. The first channel is mode, the next speed.
fn renderDMXSpinBipolar2ChWithRange(n: f64, range: &[SpinRangeMatrix<u8>], offset: u16, buffer: &[u8]) -> u8 {
    let nn = fLimitBiUnit(n);
    let (mode, speed) = (
        if nn == 0.0 { // motionless
            (range.stop.min, 0)
            // TODO: customizable speed range, in case 1 is still or 254 is fastest.
        } else if nn > 0.0 { // forward
            // TODO verify rounding
            (range.fwd.min, (nn * 255.999999) as u8)
        } else { // reverse
            (range.rev.min, (-1.0 * nn * 255.999999) as u8)
        }
    );
    buffer[offset] = mode;
    // TODO: mapped channels, in case direction and rotation are noncontiguous!
    buffer[offset + 1] = speed;
    (mode, speed)
}

// IDEA: Topo structure
//   \- blender methods (as named attributes pointing to functions)
//          \- clobber
//          \- max

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
