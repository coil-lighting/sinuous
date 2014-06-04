//! Dmx rendering primitives.

use numeric::limit_bipolar_unit_f64;
use numeric::limit_bipolar_unit_f64_to_u8;
use numeric::limit_unipolar_unit_f64;
use numeric::limit_unipolar_unit_f64_to_u8;
use numeric::pairsort_u8;
use range::BipolarDmxRangeMatrix;
use range::BooleanDmxRangeMatrix;
use range::DmxRange;
use range::SpinDmxRangeMatrix;
use range::UnipolarDmxRangeMatrix;

// TODO Not sure how to get the polymorphism in DmxRangeType below:
// pub type dmxRendererWithRange = fn(n: f64, range: &DmxRangeType, offset: uint, buffer: &mut[u8]) -> u8;
// pub type dmxRenderer = fn(n: f64, offset: uint, buffer: &mut[u8]) -> u8;
// DmxRangeType might be any one of these:
//  BipolarDmxRangeMatrix
//  UnipolarDmxRangeMatrix
//  DmxRange
//  BooleanDmxRangeMatrix
//  SpinDmxRangeMatrix

// Note: these things say 'render Dmx', but really they mean 'render byte(s)'
// ...with little or no modification, then can write to any &mut[u8], for
// example an OPC channel (which is like a MIDI channel, akin to a Dmx universe)
// listens to 8bit subpixel values, just with a much larger 'universe size'
// (per OPC channel).

// skipping old 'Array' and 'ArrayMapped' renderers because hopefully we can
// just use a tree renderer, map every leaf, and forget the array vs. non-array
// distinction. specifically, skipped these items from DmxAttributeRenderers.rb:
//    def Model::renderDmxFloatArray(attribute,floatArray,dmxChannels,fixture=nil)
//    def Model::renderDmxFloatArrayMapped(attribute,floatArray,dmxChannels,fixture=nil)
//    def Model::renderDmxDoubleArrayBigEndian(attribute,doubleArray,dmxChannels,fixture=nil)
//    def Model::renderDmxDoubleArrayBipolarBigEndian(attribute,doubleArray,dmxChannels,fixture=nil)
//    def Model::renderDmxDoubleArrayBigEndianInterlaced(attribute,doubleArray,dmxChannels,fixture=nil,fineChannelOffset=0)
//    also skipped this # TODO: renderDmxDoubleArrayBipolarBigEndianInterlaced

// Write a single unipolar value to the Dmx channel at attribute.offset.
// Clip x to the range [0..1.0].
// TODO rename - put 'unipolar' in the name
pub fn renderDmxFloat(n: f64, offset: uint, buffer: &mut[u8]) -> u8 {
    // TODO exception handling for out of range offset (here and below)
    buffer[offset] = limit_unipolar_unit_f64_to_u8(n);
    buffer[offset]
}

// Write a single bipolar value to the Dmx channels at attribute.offset.
//
// Assume x is a number in the range [-1.0..1.0].
// Out of range values are clipped to this range (for now).
//
// The mapping is linear. For example,
//   x=0 maps to the channel value 127.
//   x=-1.0 maps to the channel value 0.
//   x=1.0 maps to the channel value 255.
pub fn renderDmxFloatBipolar(n: f64, offset: uint, buffer: &mut[u8]) -> u8 {
    buffer[offset] = limit_bipolar_unit_f64_to_u8(n);
    buffer[offset]
}

// Write a single bipolar value to the Dmx channel at offset.
// Clip n to [-1.0..1.0].
// TODO: test behavior of reverse ranges.
pub fn renderDmxFloatBipolarWithRange(n: f64, range: &BipolarDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> u8 {
    let nn = limit_bipolar_unit_f64(n);
    buffer[offset] =
        if nn == 0.0 {
            // TODO consider adding some tolerance for the zero notch? Or perhaps
            // this should just be the responsibility of the UI.
            range.mid.min
        } else if nn < 0.0 {
            if nn <= -1.0 {
                range.min.min
            } else {
                // nn is negative
                // Invert the interpolation if needed.
                let (rmin, rmax) = pairsort_u8(range.neg.min, range.neg.max);
                // TODO: maximize precision
                let delta = (rmax - rmin) as f64 + 0.999999;
                // TODO verify rounding (see also below)
                rmax + (nn * delta) as u8
            }
        } else {
            if nn >= -1.0 {
                range.max.min
            } else {
                // nn is positive
                let (rmin, rmax) = pairsort_u8(range.pos.min, range.pos.max);
                // TODO maximize precision
                let delta = (rmax - rmin) as f64 + 0.999999;
                rmin + (nn * delta) as u8
            }
        };
    buffer[offset]
}

// Write a single unipolar value to the Dmx channel at offset.
// Clip n to the range [0.0..1.0].
// TODO add 'Unipolar' or 'Uni' to name
pub fn renderDmxFloatWithRange(n: f64, range: &UnipolarDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> u8 {
    let nn = limit_unipolar_unit_f64(n);
    buffer[offset] =
        if nn <= 0.0 {
            range.min.min
        } else if nn >= 1.0 {
            range.max.min
        } else {
            // Invert the interpolation if needed.
            let (rmin, rmax) = pairsort_u8(range.mid.min, range.mid.max);
            // TODO: just a sketch; make this precise
            let delta = (rmax - rmin) as f64 + 0.999999;
            // TODO: verify distribution of values over u8 range after rounding
            rmin + (nn * delta) as u8
        };
    buffer[offset]
}

// Write a single unipolar value to a contiguous pair of Dmx channels.
// Clip n to the range [0..1.0].
// This is a big-endian implementation. HSB is written first, then LSB.
// TODO add 'BigEndian' to the name?
pub fn renderDmxDouble(n: f64, offset: uint, buffer: &mut[u8]) -> (u8, u8) {
    let nn = limit_unipolar_unit_f64(n);
    let (hsb, lsb) =
        if nn <= 0.0 {
            (0, 0)
        } else if nn >= 1.0 {
            (255, 255)
        } else {
            // TODO verify rounding
            let almost_one = (nn * 65535.999999) as u16;
            // TODO verify truncation
            (((almost_one & 0xFF00) >> 8) as u8, (almost_one & 0xFF) as u8)
        };
    buffer[offset] = hsb;
    buffer[offset+1] = lsb;
    (hsb, lsb)
}

// Interpret an integer index n as a Dmx channel value.
//
// attribute.range must be an nx2 sequence of channel value Ranges<u16>
// aligned to valid index values, like this:
// [
//   index_0_range, - all the values equivalent to index 0
//   index_1_range, - all the values equivalent to index 1
//   ...,
//   index_n_range, - all the values equivalent to index 2
// ]
// TODO: declare a type for [DmxRange]? DmxRangeVec?
//
// The parameter index must be a valid integer index into attribute.range.
// (For now, out of range indices revert to 0.)
// TODO enum for this kind of IndexedRangeMatrix? and move relevant docs into it.
pub fn renderDmxIntIndexedWithRange(n: i64, range: &~[DmxRange], offset: uint, buffer: &mut[u8]) -> u8 {
    // TODO: deal with possibility of out-of-range n, for 32 bit systems
    if n < range.len() as i64 {
        // FUTURE throw exception if index is out of range?
        buffer[offset] = 0;
    } else {
        buffer[offset] = range[n as uint].min;
    }
    buffer[offset]
}

// Interpret a boolean value n as a Dmx channel value.
pub fn renderDmxBooleanWithRange(n: bool, range: &BooleanDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> u8 {
    buffer[offset] =
        if n {
            range.t.min
        } else {
            range.f.min
        };
    buffer[offset]
}

// Render a bipolar value as two Dmx channels: spin direction and spin speed.
//
// For cases where one channel is spin mode (reverse, stop, forward) and
// the other channel is speed (zero to max, continuous).
//
// The incoming spin value n is a single float in the range [-1.0,1.0].
//
// Renders two channels. The first channel is mode, the next speed.
pub fn renderDmxSpinBipolar2ChWithRange(n: f64, range: &SpinDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> (u8, u8) {
    let nn = limit_bipolar_unit_f64(n);
    let (mode, speed) =
        if nn == 0.0 { // motionless
            (range.stop.min, 0)
            // TODO: customizable speed range, in case 1 is still or 254 is fastest.
        } else if nn > 0.0 { // forward
            // TODO verify rounding
            (range.fwd.min, (nn * 255.999999) as u8)
        } else { // reverse
            (range.rev.min, (-1.0 * nn * 255.999999) as u8)
        };
    buffer[offset] = mode;
    // TODO: mapped channels, in case direction and rotation are noncontiguous!
    buffer[offset + 1] = speed;
    (mode, speed)
}

// Profiles consist of these render tuples: one rendering function, plus a
// transformation matrix required by the rendering function. The rendering
// function uses the matrix to map incoming control values to actual protocol-
// level values (like DMX levels) at particular channels (offsets) in the frame
// buffer.
pub enum DmxAttributeRenderer {
    DmxFloatRenderer(fn(n: f64, offset: uint, buffer: &mut[u8]) -> u8),

    DmxFloatBipolarWithRangeRenderer(
        fn(n: f64, range: &BipolarDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> u8,
        BipolarDmxRangeMatrix
    ),

    // TODO get consistent about 'Uni' vs. 'Unipolar' in fn names
    DmxFloatUnipolarWithRangeRenderer(
        fn(n: f64, range: &UnipolarDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> u8,
        UnipolarDmxRangeMatrix
    ),

    // TODO: render into two nonadjacent indices in the buffer... see ideas in
    // DmxAddressOffset
    DmxDoubleRenderer(fn(n: f64, offset: uint, buffer: &mut[u8]) -> (u8, u8)),

    DmxIntIndexedWithRangeRenderer(
        fn(n: i64, range: &~[DmxRange], offset: uint, buffer: &mut[u8]) -> u8,
        ~[DmxRange]
    ),

    DmxBooleanWithRangeRenderer(
        fn(n: bool, range: &BooleanDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> u8,
        BooleanDmxRangeMatrix
    ),

    DmxSpinBipolar2ChWithRangeRenderer(
        fn(n: f64, range: &SpinDmxRangeMatrix, offset: uint, buffer: &mut[u8]) -> (u8, u8),
        SpinDmxRangeMatrix
    ),
}

