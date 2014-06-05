//! Blending primitives.
//! TODO: rename these methods to Rust style once I've ported their
//! dependencies from rb. - MB
//! Derived from LD50's AttributeBlenderMethods.rb by Mike Bissell.
use numeric::limit_bipolar_unit_f64;
use numeric::limit_euclid_i64;
use numeric::limit_unipolar_unit_f64;
use numeric::median_ring_i64;
use numeric::sort_apply_f64;
use numeric::wrap_ring_i64;
use numeric::wrap_ring_bipolar_f64;
use numeric::wrap_ring_unipolar_f64;

/// Given two attribute values a and b, just return a copy of a, "clobbering" b.
pub fn iblend_clobber(a:i64, _b:i64) -> i64 {
    a
}

// TODO REF minimum, maximum to use real range structs

/// Given two attribute values within a range, return the sum of the two,
/// limited to the specified range.
pub fn iblend_euclid_add(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a + b, minimum, maximum)
}

/// Given two attribute values within a range, return the difference of the two,
/// limited to the specified range.
pub fn iblend_euclid_subtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a - b, minimum, maximum)
}

/// Return the sum of two attribute values, wrapping around the outer limits
/// specified by the given range.
pub fn iblend_ring_add(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a + b, minimum, maximum)
}

/// Return the difference of two attribute values. Wrap around the outer limits
/// specified by the given range.
pub fn iblend_ring_subtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a - b, minimum, maximum)
}

/// Return the average (median) of Euclidian inputs a and b, rounding to the
/// nearest integer.
///
/// N.B. Rounding of *.5 numbers (1.5, -1.5) follows the behavior of Rust's
/// round function, which rounds positive halves up and negative halves
/// down.
pub fn iblend_euclid_median(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(((a + b) as f64 / 2.0).round() as i64, minimum, maximum)
}

// TODO skipped: iblend_euclid_multiply (though it could be done with ints).
// (Let's wait until we can experiment and observe the aesthetic results.)

/// Return the average (median) of a and b, rounding to the nearest integer.
/// The median is defined as the closest integer to the central value dividing
/// the shortest path between a and b, wrapping around the outer limits
/// given as [minimum...maximum]. See rounding note on fn iblend_euclid_median.
pub fn iblend_ring_median(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    median_ring_i64(a, b, minimum, maximum)
}

/// Given a Euclidian value a and b, return a new value which is the maximum of
/// the two.
pub fn iblend_euclid_max(a: i64, b: i64) -> i64 {
    if a >= b {
        a
    } else {
        b
    }
}

/// Given a Euclidian value a and b, return a new value which is the minimum of
/// the two.
pub fn iblend_euclid_min(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

/// If abs(a) is greater than b, return a, else b. If a and b have the
/// same absolute value, return a.
pub fn iblend_euclid_abs_max(a: i64, b: i64) -> i64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

/// Compliment of iblend_euclid_abs_max. If a and b have the same absolute
/// value, return b.
pub fn iblend_euclid_abs_min(a: i64, b: i64) -> i64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}


// -----------------------------------------------------------------------------
// Float blenders

// TODO: some of these might be made generic on primitives like this:
// pub fn abs<T: Signed>(value: T) -> T -- but can we do it without dynamic
// lookups? Currently we must work around a 0.11-pre compiler bug by not using
// generics here.

/// Given two attribute values a and b, just return a copy of a, "clobbering" b.
pub fn fblend_clobber(a:f64, _b:f64) -> f64 {
    a
}

/// Given a Euclidian value a and b, return a new value which is the maximum of
/// the two.
pub fn fblend_euclid_max(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}

/// Given a Euclidian value a and b, return a new value which is the minimum of
/// the two.
pub fn fblend_euclid_min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

/// If abs(a) is greater than b, return a, else b. If a and b have the
/// same absolute value, return a.
pub fn fblend_euclid_abs_max(a: f64, b: f64) -> f64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

/// Compliment of fblend_euclid_abs_max. If a and b have the same absolute,
/// value, return b.
pub fn fblend_euclid_abs_min(a: f64, b: f64) -> f64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

/// Return the sum of two signed values, limited to the range [-1.0..1.0].
/// Do not wrap around the outer limits.
pub fn fblend_euclid_bi_add(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a + b)
}

/// Return the difference of two signed values, limited to the range
/// [-1.0..1.0]. Do not wrap around the outer limits.
pub fn fblend_euclid_bi_subtract(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a - b)
}

/// Return the sum of two unsigned values, limited to the range [0.0..1.0].
/// Do not wrap around the outer limits.
pub fn fblend_euclid_uni_add(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a + b)
}

/// Return the difference of two unsigned values, limited to the range
/// [0.0..1.0]. Do not wrap around the outer limits.
pub fn fblend_euclid_uni_subtract(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a - b)
}

/// Return the sum of two unsigned values, limited to the range [0.0..1.0].
/// Wrap around the outer limits.
pub fn fblend_ring_uni_add(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a + b)
}

/// Return the difference of two unsigned values, limited to the range
/// [0.0..1.0]. Wrap around the outer limits.
pub fn fblend_ring_uni_subtract(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a - b)
}

/// Return the sum of two signed values, limited to the range [-1.0..1.0].
/// Wrap around the outer limits, centering on 0.
/// Could yield strange aesthetic results.
pub fn fblend_ring_bi_add(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a + b)
}

/// Return the difference of two signed values, limited to the range
/// [-1.0..1.0]. Wrap around the outer limits, centering on 0.
/// Could yield strange aesthetic results.
pub fn fblend_ring_bi_subtract(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a - b)
}

/// Return the average (mean) of two Euclidian values. This effectively limits
/// output to [a..b], meaning this method is compatible with both signed
/// and unsigned unit Euclidian inputs.
/// TODO - We should probably actively limit this. We currently just trust that
/// a and b are in range, but better safe than sorry.
pub fn fblend_euclid_mean(a: f64, b: f64) -> f64 {
    (a + b) / 2.0
}

/// Return the result of multiplying two Euclidian values. This effectively
/// limits output to [0..1] for unsigned, unit-normalized inputs or [-1..1] for
/// signed, biunit-normalized values, as long as a and b are properly
/// normalized already. TODO - Let's just actively limit this. See above.
pub fn fblend_euclid_multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// Private impl for two functions below.
fn fblend_ring_mean(f: fn(f64) -> f64, a: f64, b: f64) -> f64 {
    let (aa, bb) = sort_apply_f64(f, a, b);
    let highRoad = 1.0 - aa + bb;
    let lowRoad = aa - bb;

    // choose the shortest route
    if highRoad < lowRoad {
        f(aa + highRoad/2.0)
    } else {
        (aa + bb) / 2.0
    }
    // TODO actively limit output?
}

/// Return the average (mean) of two unsigned ring values. The mean is the
/// central value dividing the shortest path between a and b, wrapping around
/// the outer limits. Output is in the range [0..1].
pub fn fblend_ring_uni_mean(a: f64, b: f64) -> f64 {
    fblend_ring_mean(wrap_ring_unipolar_f64, a, b)
}

/// Return the average (mean) of two signed ring values. The mean is the
/// central value dividing the shortest path between a and b, wrapping around
/// the outer limits. Output is in the range [-1..1].
pub fn fblend_ring_bi_mean(a: f64, b: f64) -> f64 {
    fblend_ring_mean(wrap_ring_bipolar_f64, a, b)
}
