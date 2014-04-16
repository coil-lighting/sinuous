//! Encode ranges of values. This is mostly used for mapping attribute values into
//! valid numeric ranges for DMX channel values.

pub struct Range <T> {
    min: T,
    max: T,
}

pub type DmxRange = Range<u8>;


// OLD notes follow. Review & merge/purge as required.
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
pub struct BipolarChannelValueRangeMatrix<T> {
    // N.B. neg.min may be greater than neg.max, or pos.min may be greater than
    // pos.max, inverting the interpolation for the respective subrange.
    // See renderDMXFloatBipolarWithRange for details.
    min: Range<T>, // Values equivalent to -1.0
    neg: Range<T>, // Values in the range (-1.0..0.0), exclusive
    mid: Range<T>, // Values equivalent to 0.0
    pos: Range<T>, // Values in the range (0.0..1.0), exclusive
    max: Range<T>, // Values equivalent to 1.0
}

pub type BipolarDmxRangeMatrix = BipolarChannelValueRangeMatrix<u8>;

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
pub struct UnipolarRangeMatrix<T> {
    // N.B. mid.min may be greater than mid.max, inverting the interpolation.
    // See renderDMXFloatWithRange for details.
    min: Range<T>, // Values equivalent to 0.0
    mid: Range<T>, // Values in the range (0.0.. 1.0), exclusive
    max: Range<T>, // Values equivalent to 1.0
}

pub type UnipolarDmxRangeMatrix = UnipolarRangeMatrix<u8>;

// TODO: should we make an enum to name the following pattern?
// attribute.range must be an nx2 sequence of channel value Ranges<u16>
// aligned to valid index values, like this:
// [
//   index_0_range, - all the values equivalent to index 0
//   index_1_range, - all the values equivalent to index 1
//   ...,
//   index_n_range, - all the values equivalent to index 2
// ]
// see range: &[Range<u8>] in renderDMXIntIndexedWithRange

// This range matrix allows us to interpret the sign of a spin value as a DMX
// channel level (reverse for negative, stop for 0, fwd for positive).
//
// The magnitude of value is interpreted as speed. Currently speed renders
// linearly from 0 (still) to 255 (fastest). See below.
pub struct SpinRangeMatrix<T> {
    rev:  Range<T>, // Reverse values, slow through fast
    stop: Range<T>, // Values equivalent to stationary
    fwd:  Range<T>, // Forward values, slow through fast
}

pub type SpinDmxRangeMatrix = SpinRangeMatrix<u8>;

pub struct BooleanRangeMatrix<T> {
    f: Range<T>, // Values equivalent to false
    t: Range<T>, // Values equivalent to true
}

pub type BooleanDmxRangeMatrix = BooleanRangeMatrix<u8>;
