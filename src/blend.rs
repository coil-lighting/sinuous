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
pub fn iblendClobber(a:i64, _b:i64) -> i64 {
    a
}

pub fn iblendEuclidAdd(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a + b, minimum, maximum)
}

pub fn iblendEuclidSubtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a - b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
pub fn iblendRingAdd(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a + b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
pub fn iblendRingSubtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a - b, minimum, maximum)
}

// Return the average of a and b, rounding to the nearest integer.
// Rounding of *.5 numbers (1.5, -1.5) follows the behavior of Rust's
// round function, which rounds positive halves up and negative halves
// down.
pub fn iblendEuclidMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(((a + b) as f64 / 2.0).round() as i64, minimum, maximum)
}

// TODO skipped: iblendEuclidMultiply (though it could be done).
// (Let's wait until we can practically experiment.)

pub fn iblendRingMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    median_ring_i64(a, b, minimum, maximum)
}

// Given a Euclidian value a and b, return a new value which is the maximum of
// the two.
pub fn iblendEuclidMax(a: i64, b: i64) -> i64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
pub fn iblendEuclidMin(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
pub fn iblendEuclidAbsMax(a: i64, b: i64) -> i64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of iblendEuclidAbsMax. If a and b have the same absolute value,
// return b.
pub fn iblendEuclidAbsMin(a: i64, b: i64) -> i64 {
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
pub fn fblendClobber(a:f64, _b:f64) -> f64 {
    a
}

// Given a Euclidian value a and b, return a new value which is the maximum of
// the two.
pub fn fblendEuclidMax(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
pub fn fblendEuclidMin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
pub fn fblendEuclidAbsMax(a: f64, b: f64) -> f64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of fblendEuclidAbsMax. If a and b have the same absolute value,
// return b.
pub fn fblendEuclidAbsMin(a: f64, b: f64) -> f64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

pub fn fblendEuclidBiAdd(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a + b)
}

pub fn fblendEuclidBiSubtract(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a - b)
}

pub fn fblendEuclidUniAdd(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a + b)
}

pub fn fblendEuclidUniSubtract(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a - b)
}

pub fn fblendRingUniAdd(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a + b)
}

pub fn fblendRingUniSubtract(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a - b)
}

// If a + b is out of range, wrap it. centers around 0. Could be really weird.
pub fn fblendRingBiAdd(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a + b)
}

// If a - b is out of range, wrap it. centers around 0. Could be really weird.
pub fn fblendRingBiSubtract(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a - b)
}

pub fn fblendEuclidMedian(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    (a + b) / 2.0
}

pub fn fblendEuclidMultiply(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    a * b
}

// private impl for 2 fns below. TODO: rename
pub fn _fblendRingMedian(f: fn(f64) -> f64, a: f64, b: f64) -> f64 {
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

pub fn fblendRingUniMedian(a: f64, b: f64) -> f64 {
    _fblendRingMedian(wrap_ring_unipolar_f64, a, b)
}

pub fn fblendRingBiMedian(a: f64, b: f64) -> f64 {
    _fblendRingMedian(wrap_ring_bipolar_f64, a, b)
}
