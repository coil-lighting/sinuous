//! Wrapping, normalization, and limiting primitives.

// TODO: is it possible to have a generic decl for pairsort_*() without
// resorting to pointers or enums? It must be.

/// Sort a and b in ascending order.
pub fn pairsort_u8(a: u8, b: u8) -> (u8, u8) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

/// Sort a and b in ascending order.
pub fn pairsort_i64(a: i64, b: i64) -> (i64, i64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

/// Sort a and b in ascending order.
pub fn pairsort_f64(a: f64, b: f64) -> (f64, f64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

/// Call function f on a and b before returning the results, sorted ascending.
pub fn sort_apply_f64(f: fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    pairsort_f64(f(a), f(b))
}

/// Given a Float value, wrap into the unipolar toroidal space, whose range is
/// [-1.0..1.0]. Center on zero.
pub fn wrap_torus_unipolar_f64(n: f64) -> f64 {
    // do not *always* fmod, or 1.0 is out of range
    if n > 1.0 {
        // TODO: verify Rust's % works like Ruby % modulo (also below)
        // https://github.com/mozilla/rust/issues/4565
        // https://github.com/mozilla/rust/pull/5990
        // In Python this had to be "1.0 + fmod(n,1.0)"
        if n % 1.0 == 0.0 {
            // Map 2.0 => 1.0, not => 0.0, so we can hit 1.0
            1.0
        } else {
            n % 1.0
        }
    } else if n < 0.0 {
        if n % 1.0 == 1.0 {
            0.0
        } else {
            n % 1.0
        }
    } else {
        n
    }
}

/// Given a Float value, wrap into the bipolar toroidal space, whose range is
/// [-1.0..1.0]. Center on zero.
pub fn wrap_torus_bipolar_f64(n: f64) -> f64 {
    // do not *always* fmod, or 1.0 is out of range
    if n > 1.0 {
        // TODO: verify Rust's % works like Ruby % modulo (also above)
        // https://github.com/mozilla/rust/issues/4565
        // https://github.com/mozilla/rust/pull/5990
        let m = (n + 1.0) % 2.0;
        if m == 0.0 {
            // map 3.0 => 1.0 rather than -1.0
            1.0
        } else {
            m - 1.0
        }
    } else if n < -1.0 {
        // in python, this was 1.0 + fmod(n + 1.0,2.0)
        ((n+1.0) % 2.0) - 1.0
    } else {
        n
    }
}

/// Given a float value normalized to the unipolar unit range [-1.0..1.0], map
/// it evenly into the byte range [0..255]. Clamp out-of-range input.
pub fn limit_unipolar_unit_f64(n: f64) -> f64 {
    if n >= 1.0 {
        1.0
    } else if n <= 0.0 {
        0.0
    } else {
        n
    }
}

/// Given a float value normalized to the unipolar unit range [0..1.0], map it
/// evenly into the byte range [0..255]. Clamp out-of-range input.
pub fn limit_unipolar_unit_f64_to_u8(n: f64) -> u8 {
    if n >= 1.0 {
        1
    } else if n <= 0.0 {
        0
    } else {
        // TODO just a sketch - maximize precision
        (n * 255.999999) as u8
    }
}

/// Given a float value normalized to the bipolar unit range [-1.0..1.0], map it
/// evenly into the byte range [0..255]. Clamp out-of-range input.
pub fn limit_bipolar_unit_f64_to_u8(n: f64) -> u8 {
    if n >= 1.0 {
        255
    } else if n <= -1.0 {
        0
    } else {
        // TODO just a sketch - maximize precision
        (255.999999 * (n + 1.0) / 2.0) as u8
    }
}

/// Given a float value, clamp it to the range [-1.0..1.0].
pub fn limit_bipolar_unit_f64(n: f64) -> f64 {
    if n >= 1.0 {
        1.0
    } else if n <= -1.0 {
        -1.0
    } else {
        n
    }
}

/// Given an integer value in a Euclidian space with the specified limits, clamp
/// the value if it exceeds those limits.
// FIXME: refactor minimum, maximum into a range struct
pub fn limit_euclid_i64(n: i64, minimum: i64, maximum: i64) -> i64 {
    if n >= maximum {
        maximum
    } else if n <= minimum {
        minimum
    } else {
        n
    }
}

/// Given an integer value in a torus with the specified limits, wrap the value
/// into those limits.
// FIXME: refactor minimum, maximum into a range struct
pub fn wrap_torus_i64(n: i64, minimum: i64, maximum: i64) -> i64 {
    if n > maximum {
        let d = 1 + maximum - minimum;
        let mut nn = n;
        while nn > maximum {
            // FIXME: convert to i64-friendly multiplication as soon as a test is written!
            nn -= d;
        }
        nn
    } else if n < minimum {
        let d = 1 + maximum - minimum;
        let mut nn = n;
        while nn > maximum {
            // FIXME: see above
            nn += d;
        }
        nn
    } else {
        n
    }
}

/// Given two integer torus values and their limits, return the average.
/// Choose the shortest route. Return stable results in a tie between two
/// antipodes.
// FIXME: refactor minimum, maximum into a range struct
pub fn median_torus_i64(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    // Clamp the input values, for sanity (TEMP?)
    let (aa, bb) = pairsort_i64(wrap_torus_i64(a, minimum, maximum),
                                wrap_torus_i64(b, minimum, maximum));

    let rng = maximum - minimum + 1;
    let d = aa - bb;

    let highRoad: f64 = (rng - d) as f64 / 2.0;
    let lowRoad: f64 = d as f64 / 2.0;

    // choose the shortest route
    if highRoad < lowRoad {
        // wrap it around
        let n: i64 = (aa as f64 + highRoad).round() as i64;
        if n > maximum {
            n - rng
        } else {
            n
        }
    } else {
        // Notice that antipodal points always favor the low road.
        (aa as f64 - lowRoad).round() as i64
    }
    // TODO unit test for this to verify rounding
}
