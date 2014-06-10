//! Types to help locate devices in the real world (or a fictional 3D scene).

/// A Device's cartesian position in space.
pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

/// A Device's orientation in space.
pub struct Orientation {
    pan: f64,  // yaw
    tilt: f64, // pitch
    roll: f64, // roll
}

/// A Device's oriented position in space.
pub struct Loc {
    position: Position,
    orientation: Orientation,
}


impl Loc {
    pub fn new_origin() -> Loc {
        Loc {
            position: Position{x:0., y:0., z:0.},
            orientation: Orientation{pan: 0., tilt:0., roll:0.}
        }
    }
}

