//! Types to help locate devices in the real world (or a fictional 3D scene).

pub struct Position {
    x: f64,
    y: f64,
    z: f64,
}

pub struct Orientation {
    pan: f64,  // yaw
    tilt: f64, // pitch
    roll: f64, // roll
}

pub struct Loc {
    position: Position,
    orientation: Orientation,
}
