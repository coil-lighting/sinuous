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

extern crate collections;

use std::f64;
use std::u64;
use collections::HashMap;

// using enums as unions for now, hopefully this is ok. TODO: check memory layout

enum ProfileElement {
    Profile(Profile),
    Attribute(Attribute),
}

// TODO determine whether we could use the topo types themselves as values in
// the topology array
enum Topo {
    // [0.0,1.0] Naturally continuous, values bounded, interpolation recommended (e.g. dimmer).
    VectorEuclidianUnipolar=0,

    // [-1.0,1.0] (e.g. XY position)
    VectorEuclidianBipolar,

    // [0.0,1.0] Naturally continuous, values wrap, interpolation recommended (e.g. angle of rotation).
    VectorToroidalUnipolar,

    // [-1.0,1.0] Might come in handy if there is a natural neutral '0' point in a torus -- EXPERIMENTAL
    VectorToroidalBipolar,

    // Int indexed from 0. Naturally discontinuous, values wrap, interpolation optionally mechanically/logically meaningful but discouraged (e.g. litho index).
    DiscreteRing,

    // Int indexed from 0. Naturally discontinuous, values bounded, interpolation optionally mechanically/logically meaningful but discouraged (e.g. linear slide tray index).
    DiscreteArray,

    // Int indexed from 0. Naturally discontinuous, values bounded, interpolation mechanically/logically meaningless and therefore discouraged (e.g. color wheel mode).
    DiscreteSet,

    // Topology is undefined, probably because this is a (virtual?) cluster parent.
    UndefinedTopo,
}

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

enum AttributeType {
    // name                     *dmx* renderer
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
      Position          =  10, // dims separately specified
      Orientation       =  11, // ditto
      Transform         =  12, // ditto
      FilterSelect      =  13, // e.g. litho index
      FilterIntensity   =  14, // e.g. frost (should frost get its own filter type?)
      Raster            =  15,
      Control           =  16,
      Smoothing         =  17,
}

// Effect subtypes - where needed, dimensionality is separately specified.
// TODO Make sure there is a test that verifies that all these have impls.
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

struct DmxMap {
    addresses: ~[u8], // 1 or more, relative to profile not universe - TODO constrain positive
    offset: ~[DmxAddressOffset],
    ranges: ~[DmxValueRange],
}

struct Attribute {
    name: ~str, // e.g. "iris"
    topology: ~[Topo],

    // Specify the number of dimensions in this Attribute.
    // A 2D attribute (x,y) has dimensionality=2.
    // A 1D attribute (dimmer level) has dimensionality=1.
    dimensionality: u32,
    effect: (EffectType, EffectSubtype, EffectSubsubtype),
    virtual: bool, // TODO is this needed? might be able to make it virtual by enabling/disabling peer subdevices
    attribute_type: AttributeType,
    dmx: DmxMap, // or null

    // OLD instructions. for simplicity, I think just making it always an array 
    // for now will do. If it feels inconvenient we might accept singles through
    // a polymorphic setter function, not in the underlying memory layout.
    // # default -- The default value for this Attribute. For multidimensional
    // # attributes (>1D), default must be a *sequence* of len(dims). For one
    // # dimensional attributes, default must a valid value, not a sequence
    // # whose len is 1. (This allows us to skip many array lookups for most
    // # Attributes.) This value is stored as .default. For multidimensional
    // # attributes, we convert this value to a list before storing it so
    // # that Fixture can simply deepcopy its contents upon initialization.
    default: ~[AttributeValue],

    // from rb, maybe no longer needed:
    // ONLY IF THIS ATTRIBUTE NEEDS LABELED INDICES
    //     @indexTable=nil // if required, a tuple of strings, one per mode, in order

    // ONLY IF THIS ATTRIBUTE GOVERNS A MODAL CLUSTER
    //     @clusterMethod=nil

}

// TODO: version, device author
struct Profile {
    name: ~str, // "Technobeam"
    nickname: ~str, // "Techno"
    creator_name: ~str, // "HES"
    version: int, // 1
    elements: ~[ProfileElement],
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
    // A Device has multiple addrs in case one logical device manages more than 
    // one physical device, each potentially with its own address.
    addr: Addr, 

    // A Device has multiple locations in case more than one physical device,
    // potentially with different addresses, is managed by one logical device.
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


struct Device {
    profile: Profile,
    name: ~str,
    nickname: ~str, // shorter, to save space (defaults to name, truncated)
    patches: ~[DevicePatch],
}

fn main() {
    println!("Done.")
}
