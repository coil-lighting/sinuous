//! Types describing effects, and eventually functions to help the UI draw them.

// Matrix-mappable effect ((sub)sub)type (hint) metadata (EXPERIMENTAL).
// Conceptually, types, subtypes, and subsubtypes exist in a 3D space of
// options. Currently there are about 1000 option combinations total (see
// FECT_MAXIMA). Many combinations don't actually make sense from a design
// standpoint.
//
// From Ruby (probably no longer needed):
// the maximum size of a 3D array required to map all effects combos
//     FECT_MAXIMA = [17,27,2].freeze
//
// These type hints must not impact DMX rendering! They may impact the
// display of previews, icons, or controls in your onscreen controller.
//
// In ruby, I put these types in order so they could be used as array indices,
// and to stabilize their display order. Rust might give us a better way to
// do this.
//
// FIXME these probably should be structs, not constants. they were constants
// because of the way they were wired up in Python.
pub enum EffectType {
    Misc              =   0,
    ModeSelect        =   1,
    Color             =   2,
    Dimmer            =   3,
    Relay             =   4,
    Focus             =   5,
    Zoom              =   6,
    Iris              =   7,
    Frame             =   8,
    Strobe            =   9,
    Position          =  10,
    Orientation       =  11,
    Transform         =  12,
    FilterSelect      =  13, // e.g. litho index
    FilterIntensity   =  14, // e.g. frost (should frost get its own filter type?)
    Raster            =  15,
    Control           =  16,
    Smoothing         =  17,
}

// Effect subtypes - where needed, dimensionality is separately specified.
//
// TODO Make sure there is a test that verifies that all these have impls.
//
// FIXME these probably should be structs, not constants. they were constants
// because of the way they were wired up in Python.
pub enum EffectSubtype {
    Other               =  0,
    ColorspaceRgb       =  1, // red + green + blue
    ColorspaceHsb       =  2, // hue + saturation + brightness
    ColorspaceRgbi      =  3, // i = intensity multiplied
    ColorspaceRgbw      =  4, // w = white added
    ColorspaceRgbaw     =  5, // a = amber added
    ColorspaceHsl       =  6,
    Colorspace1x        =  7, // one mono filter such as a dichro
    Colorspace2x        =  8, // two mono filters
    Colorspace3x        =  9, // etc.
    Colorspace4x        = 10,
    Colorspace5x        = 11,
    ColorspaceI1x       = 12, // intensity + one mono filter such as a dichro
    ColorspaceI2x       = 13, // intensity + two mono filters
    ColorspaceI3x       = 14, // etc.
    ColorspaceI4x       = 15,
    ColorspaceI5x       = 16,
    ColorspaceI         = 17, // just intensity, a.k.a. grayscale or dimmer
    TransformRotate     = 18,
    TransformScroll     = 19,
    TransformTranslate  = 20,
    FilterMisc          = 21, // e.g. an effects wheel where each slot does something different
    FilterMultiply      = 22, // e.g. prism, pyramid mirror, barrel mirror, moonflower mirror, disco ball
    FilterDistort       = 23, // e.g. textured glass
    FilterSubtract      = 24, // e.g. vignette, gobo
    FilterAdd           = 25, // e.g. laser on technobeam
    OrientationMirror   = 26,
    OrientationYoke     = 27,
}

// TODO add frame leaf order / orientation? NSEW, NESW... (N=North, S=South...)
// Or just impose a normal leaf order and allow offset map to scramble it?

pub enum EffectSubsubtype {
    Value     = 0, // e.g. intensity, must be 0
    Frequency = 1, // e.g. rotation speed
    Duration  = 2, // e.g. mspeed smoothing time
}
