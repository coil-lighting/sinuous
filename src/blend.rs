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

// Given two attribute values a and b, just return a copy of a, "clobbering" b.
pub fn iblend_clobber(a:i64, _b:i64) -> i64 {
    a
}

pub fn iblend_euclidAdd(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a + b, minimum, maximum)
}

pub fn iblend_euclid_subtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a - b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
pub fn iblend_ring_add(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a + b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
pub fn iblend_ring_subtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a - b, minimum, maximum)
}

// Return the average of a and b, rounding to the nearest integer.
// Rounding of *.5 numbers (1.5, -1.5) follows the behavior of Rust's
// round function, which rounds positive halves up and negative halves
// down.
pub fn iblend_euclid_median(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(((a + b) as f64 / 2.0).round() as i64, minimum, maximum)
}

// TODO skipped: iblend_euclid_multiply (though it could be done).
// (Let's wait until we can practically experiment.)

pub fn iblend_ring_median(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    median_ring_i64(a, b, minimum, maximum)
}

// Given a Euclidian value a and b, return a new value which is the maximum of
// the two.
pub fn iblend_euclid_max(a: i64, b: i64) -> i64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
pub fn iblend_euclid_min(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
pub fn iblend_euclid_abs_max(a: i64, b: i64) -> i64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of iblend_euclid_abs_max. If a and b have the same absolute value,
// return b.
pub fn iblend_euclid_abs_min(a: i64, b: i64) -> i64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
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
// Work around a 0.11-pre compiler bug by not using generics.

// Given two attribute values a and b, just return a copy of a, "clobbering" b.
pub fn fblend_clobber(a:f64, _b:f64) -> f64 {
    a
}

// Given a Euclidian value a and b, return a new value which is the maximum of
// the two.
pub fn fblend_euclid_max(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
pub fn fblend_euclid_min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
pub fn fblend_euclidAbsMax(a: f64, b: f64) -> f64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of fblend_euclidAbsMax. If a and b have the same absolute value,
// return b.
pub fn fblend_euclidAbsMin(a: f64, b: f64) -> f64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

pub fn fblend_euclid_bi_add(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a + b)
}

pub fn fblend_euclid_bi_subtract(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a - b)
}

pub fn fblend_euclid_uni_add(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a + b)
}

pub fn fblend_euclid_uni_subtract(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a - b)
}

pub fn fblend_ring_uni_add(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a + b)
}

pub fn fblend_ring_uni_subtract(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a - b)
}

// If a + b is out of range, wrap it. centers around 0. Could be really weird.
pub fn fblend_ring_bi_add(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a + b)
}

// If a - b is out of range, wrap it. centers around 0. Could be really weird.
pub fn fblend_ring_bi_subtract(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a - b)
}

pub fn fblend_euclid_median(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    (a + b) / 2.0
}

pub fn fblend_euclid_multiply(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    a * b
}

// private impl for 2 fns below. TODO: rename
pub fn _fblend_ring_median(f: fn(f64) -> f64, a: f64, b: f64) -> f64 {
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

pub fn fblend_ring_uni_median(a: f64, b: f64) -> f64 {
    _fblend_ring_median(wrap_ring_unipolar_f64, a, b)
}

pub fn fblend_ring_bi_median(a: f64, b: f64) -> f64 {
    _fblend_ring_median(wrap_ring_bipolar_f64, a, b)
}
