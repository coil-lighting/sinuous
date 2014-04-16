// -----------------------------------------------------------------------------
// Low-level wrapping, normalization, and limiting functions

// TODO: is it possible to have a generic decl for pairsort_*64 without using
// pointers? It must be.

fn pairsort_u8(a: u8, b: u8) -> (u8, u8) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}


// reorder (a, b) to (b, a) if b > a
fn pairsort_i64(a: i64, b: i64) -> (i64, i64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

// reorder (a, b) to (b, a) if b > a
fn pairsort_f64(a: f64, b: f64) -> (f64, f64) {
    if a < b {
        (b, a)
    } else {
        (a, b)
    }
}

// call f on a and b before returning the sorted results
fn sort_apply_f64(f: fn(f64) -> f64, a: f64, b: f64) -> (f64, f64) {
    pairsort_f64(f(a), f(b))
}

// Given a toroidal Float value, wrap it around if it is out of the range 0...1.0 (incl.).
fn fVecTorusUniWrap(n: f64) -> f64 {
    // do not *always* fmod, or 1.0 is out of range
    if n > 1.0 {
        // TODO: verify Rust's % works like Ruby % modulo
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

// Given a toroidal Float value, wrap it around if it is out of the range -1.0...1.0 (incl.).
fn fVecTorusBiWrap(n: f64) -> f64 {
    // do not *always* fmod, or 1.0 is out of range
    if n > 1.0 {
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

fn unipolar_unit_limit_f64_to_u8(n: f64) -> u8 {
    if n >= 1.0 {
        1
    } else if n <= 0.0 {
        0
    } else {
        // TODO just a sketch - maximize precision
        (n * 255.999999) as u8
    }
}

fn fLimitUniUnit(n: f64) -> f64 {
    if n >= 1.0 {
        1.0
    } else if n <= 0.0 {
        0.0
    } else {
        n
    }
}

fn bipolar_unit_limit_f64_to_u8(n: f64) -> u8 {
    if n >= 1.0 {
        1
    } else if n <= -1.0 {
        -1
    } else {
        // TODO just a sketch - maximize precision
        (255.999999 * (n + 1.0) / 2.0) as u8
    }
}

fn fLimitBiUnit(n: f64) -> f64 {
    if n >= 1.0 {
        1.0
    } else if n <= -1.0 {
        -1.0
    } else {
        n
    }
}

fn iVecEuclidLimit(n: i64, minimum: i64, maximum: i64) -> i64 {
    if n >= maximum {
        maximum
    } else if n <= minimum {
        minimum
    } else {
        n
    }
}

fn iVecTorusWrap(n: i64, minimum: i64, maximum: i64) -> i64 {
    if n > maximum {
        let d = 1 + maximum - minimum;
        let mut nn = n;
        while nn > maximum {
            // FIXME: convert to i64-friendly multiplication after a test is written
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

fn iVecTorusMedian(a: i64, b: i64, minimum: i64, maximum: i64) -> i64 {
    // Clamp the input values, for sanity (TEMP?)
    let (aa, bb) = pairsort_i64(iVecTorusWrap(a, minimum, maximum),
                                   iVecTorusWrap(b, minimum, maximum));

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
        (aa as f64 - lowRoad).round() as i64
    }
    // TODO unit test for this to verify rounding
}
