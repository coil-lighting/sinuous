// -----------------------------------------------------------------------------
// first stab at porting AttributeBlenderMethods.rb



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
