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

// Write a single unipolar value to the DMX channel at attribute.offset.
// Clip x to the range [0..1.0].
// TODO rename - put 'unipolar' in the name
fn renderDMXFloat(n: f64, offset: u16, buffer: &mut[u8]) -> u8 {
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
fn renderDMXFloatBipolar(n: f64, offset: u16, buffer: &mut[u8]) -> u8 {
    buffer[offset] = bipolar_unit_limit_f64_to_u8(n);
    buffer[offset]
}

// Write a single bipolar value to the DMX channel at offset.
//
// Clip values to [-1.0..1.0].
//
//
// TODO: finish, test behavior of reverse ranges. Looking at the code for
// the *WithRange methods, it seems like I just 'heal' them instead of
// properly interpreting them.

fn renderDMXFloatBipolarWithRange(n: f64, range: &BipolarChannelValueRangeMatrix<u8>, offset: u16, buffer: &mut[u8]) -> u8 {
    let nn = fLimitBiUnit(n);
    buffer[offset] = (
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
        }
    );
    buffer[offset]
}

// Write a single unipolar value to the DMX channel at offset.
//
// Clip nn to the range [0.0..1.0].
//

fn renderDMXFloatWithRange(n: f64, range: &UnipolarChannelValueRangeMatrix<u8>, offset: u16, buffer: &mut[u8]) -> u8 {
    let nn = fLimitUniUnit(n);
    buffer[offset] = (
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
        }
    );
    buffer[offset]
}

// Write a single unipolar value to a contiguous pair of DMX channels.
// Clip n to the range [0..1.0].
// This is a big-endian implementation. HSB is written first, then LSB.
// TODO add 'BigEndian' to the name?
fn renderDMXDouble(n: f64, offset: u16, buffer: &mut[u8]) -> (u8, u8) {
    let nn = fLimitUniUnit(n);
    let (hsb, lsb) = (
        if nn <= 0.0 {
            (0, 0)
        } else if nn >= 1.0 {
            (255, 255)
        } else {
            // TODO verify rounding
            let almost_one = (nn * 65535.999999) as u16;
            // TODO verify truncation
            (((almost_one & 0xFF00) >> 8) as u8, (almost_one & 0xFF) as u8)
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
fn renderDMXIntIndexedWithRange(n: u16, range: &[Range<u8>], offset: u16, buffer: &mut[u8]) -> u8 {
    if n < range.len() as u16{
        // FUTURE throw exception if index is out of range?
        buffer[offset] = 0;
    } else {
        buffer[offset] = range[n].min;
    }
    buffer[offset]
}

// Interpret a boolean value n as a DMX channel value.
fn renderDMXBooleanWithRange(n: bool, range: &BooleanRangeMatrix<u8>, offset: u16, buffer: &mut[u8]) -> u8 {
    buffer[offset] = (
        if n {
            range.t.min
        } else {
            range.f.min
        }
    );
    buffer[offset]
}

// Render a bipolar value as two DMX channels: spin direction and spin speed.
//
// For cases where one channel is spin mode (reverse, stop, forward) and
// the other channel is speed (zero to max, continuous).
//
// The incoming spin value n is a single float in the range [-1.0,1.0].
//
// Renders two channels. The first channel is mode, the next speed.
fn renderDMXSpinBipolar2ChWithRange(n: f64, range: &SpinRangeMatrix<u8>, offset: u16, buffer: &mut[u8]) -> (u8, u8) {
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
