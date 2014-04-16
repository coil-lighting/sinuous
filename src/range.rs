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
struct SpinRangeMatrix<T> {
    rev:  Range<T>, // Reverse values, slow through fast
    stop: Range<T>, // Values equivalent to stationary
    fwd:  Range<T>, // Forward values, slow through fast
}

struct BooleanRangeMatrix<T> {
    f: Range<T>, // Values equivalent to false
    t: Range<T>, // Values equivalent to true
}
