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

pub fn iblendVecEuclidAdd(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a + b, minimum, maximum)
}

pub fn iblendVecEuclidSubtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(a - b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
pub fn iblendVecRingAdd(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a + b, minimum, maximum)
}

// this could be simplified for cases where minimum is always 0.
pub fn iblendVecRingSubtract(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    wrap_ring_i64(a - b, minimum, maximum)
}

// Return the average of a and b, rounding to the nearest integer.
// Rounding of *.5 numbers (1.5, -1.5) follows the behavior of Rust's
// round function, which rounds positive halves up and negative halves
// down.
pub fn iblendVecEuclidMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    limit_euclid_i64(((a + b) as f64 / 2.0).round() as i64, minimum, maximum)
}

// TODO skipped: iblendVecEuclidMultiply (though it could be done).
// (Let's wait until we can practically experiment.)

pub fn iblendVecRingMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    median_ring_i64(a, b, minimum, maximum)
}

// Given a Euclidian value a and b, return a new value which is the maximum of
// the two.
pub fn iblendVecEuclidMax(a: i64, b: i64) -> i64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
pub fn iblendVecEuclidMin(a: i64, b: i64) -> i64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
pub fn iblendVecEuclidAbsMax(a: i64, b: i64) -> i64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of iblendVecEuclidAbsMax. If a and b have the same absolute value,
// return b.
pub fn iblendVecEuclidAbsMin(a: i64, b: i64) -> i64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

// -----------------------------------------------------------------------------
// Generic blenders

// Given two attribute values a and b, just return a copy of a, "clobbering" b.
pub fn blendClobber<T>(a: T, _b: T) -> T {
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
pub fn fblendVecEuclidMax(a: f64, b: f64) -> f64 {
    if a >= b {
        a
    } else {
        b
    }
}

// Given a Euclidian value a and b, return a new value which is the minimum of
// the two.
pub fn fblendVecEuclidMin(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}

// If abs(a) is greater than b, return a, else b. If a and b have the
// same absolute value, return a.
pub fn fblendVecEuclidAbsMax(a: f64, b: f64) -> f64 {
    if a.abs() >= b.abs() {
        a
    } else {
        b
    }
}

// Compliment of fblendVecEuclidAbsMax. If a and b have the same absolute value,
// return b.
pub fn fblendVecEuclidAbsMin(a: f64, b: f64) -> f64 {
    if a.abs() < b.abs() {
        a
    } else {
        b
    }
}

pub fn fblendVecEuclidBiAdd(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a + b)
}

pub fn fblendVecEuclidBiSubtract(a: f64, b: f64) -> f64 {
    limit_bipolar_unit_f64(a - b)
}

pub fn fblendVecEuclidUniAdd(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a + b)
}

pub fn fblendVecEuclidUniSubtract(a: f64, b: f64) -> f64 {
    limit_unipolar_unit_f64(a - b)
}

pub fn fblendVecRingUniAdd(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a + b)
}

pub fn fblendVecRingUniSubtract(a: f64, b: f64) -> f64 {
    wrap_ring_unipolar_f64(a - b)
}

// If a + b is out of range, wrap it. centers around 0. Could be really weird.
pub fn fblendVecRingBiAdd(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a + b)
}

// If a - b is out of range, wrap it. centers around 0. Could be really weird.
pub fn fblendVecRingBiSubtract(a: f64, b: f64) -> f64 {
    wrap_ring_bipolar_f64(a - b)
}

pub fn fblendVecEuclidMedian(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    (a + b) / 2.0
}

pub fn fblendVecEuclidMultiply(a: f64, b: f64) -> f64 {
    // TODO - should we limit this? currently trusting that a and b are in range
    a * b
}

// private impl for 2 fns below. TODO: rename
pub fn _fblendVecRingMedian(f: fn(f64) -> f64, a: f64, b: f64) -> f64 {
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

pub fn fblendVecRingUniMedian(a: f64, b: f64) -> f64 {
    _fblendVecRingMedian(wrap_ring_unipolar_f64, a, b)
}

pub fn fblendVecRingBiMedian(a: f64, b: f64) -> f64 {
    _fblendVecRingMedian(wrap_ring_bipolar_f64, a, b)
}
