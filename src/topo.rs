use blend::fblend_clobber;
use blend::fblend_euclid_max;
use blend::fblend_euclid_min;
use blend::fblend_euclid_mean;
use blend::fblend_ring_uni_mean;
use blend::fblend_ring_bi_mean;
use blend::fblend_euclid_multiply;
use blend::fblend_euclid_uni_add;
use blend::fblend_ring_uni_add;
use blend::fblend_euclid_bi_add;
use blend::fblend_ring_bi_add;
use blend::fblend_euclid_uni_subtract;
use blend::fblend_ring_uni_subtract;
use blend::fblend_euclid_bi_subtract;
use blend::fblend_ring_bi_subtract;

use blend::iblend_clobber;
use blend::iblend_euclid_max;
use blend::iblend_euclid_min;
use blend::iblend_euclid_median;
use blend::iblend_ring_median;
use blend::iblend_ring_add;
use blend::iblend_euclid_add;
use blend::iblend_ring_subtract;
use blend::iblend_euclid_subtract;

/// A function that blends two floating-point values and returns the result.
type Blendf = fn(a:f64, b:f64) -> f64;

/// A function that blends two integer values and returns the result.
type Blendi = fn(a:i64, b:i64) -> i64;
// TODO remember why I decided that range was essential with these, or simplify.
// Maybe every int blender should take a range, for uniformity.
/// A function that blends two integer values and an integer range, returning
/// the result.
type BlendRangedi = fn(a:i64, b:i64, minimum: i64, maximum: i64) -> i64;

/// Map each high-level aesthetic blending intent to a specific implementation.
struct ContinuousBlenderTable {
    clobber: Blendf,
    max: Blendf,
    min: Blendf,
    median: Blendf,
    add: Blendf,
    subtract: Blendf,
    add_modulus: Blendf,
    subtract_modulus: Blendf,
    multiply: Blendf,
    abs_max: Blendf,
    abs_min: Blendf,
}

struct DiscreteBlenderTable {
    clobber: Blendi,
    max: Blendi,
    min: Blendi,
    median: BlendRangedi,
    add: BlendRangedi,
    subtract: BlendRangedi,
    add_modulus: BlendRangedi,
    subtract_modulus: BlendRangedi,
    multiply: Blendi,
    abs_max: Blendi,
    abs_min: Blendi,
}

/// A static lookup table mapping high-level (aesthetic) blend modes to
/// specific implementations.
enum BlenderTable {
    // there must be a less awkward way to do this...
    ContinuousBlenders(ContinuousBlenderTable),
    DiscreteBlenders(DiscreteBlenderTable),
}

/// Topology descriptors constrain and shape the parametric space for all
/// attribute values.
pub struct Topo {
    /// Quasi-continuous (encoded as f64) or discrete (encoded as i64).
    continuous: bool,

    /// Hint: interpolation aesthetically encouraged.
    blend_encouraged: bool,

    /// Hint: interpolation logically or mechanically meaningful.
    blend_meaningful: bool,

    /// Blender functions appropriate to blending pairs of values with this
    /// topology.
    blenders: BlenderTable,
}

/// Naturally continuous, values bounded, interpolation recommended.
/// No wrap-around.
/// Range: [0.0,1.0]
/// Example: dimmer
pub static continuous_euclidian_unipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_euclid_mean,
        add: fblend_euclid_uni_add,
        subtract: fblend_euclid_uni_subtract,
        add_modulus: fblend_ring_uni_add,
        subtract_modulus: fblend_ring_uni_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

/// Naturally continuous, values bounded, interpolation recommended.
/// No wrap-around. Center on 0.
/// Range: [-1.0,1.0]
/// Example: X- or Y-position on a bounded pivot or linear track
pub static continuous_euclidian_bipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_euclid_mean,
        add: fblend_euclid_bi_add,
        subtract: fblend_euclid_bi_subtract,
        add_modulus: fblend_ring_bi_add,
        subtract_modulus: fblend_ring_bi_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

/// Naturally continuous, values wrap around the outer limits, interpolation
/// recommended.
/// Range: [0.0,1.0]
/// Example: angle of rotation
pub static continuous_ring_unipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_ring_uni_mean,
        add: fblend_ring_uni_add,
        subtract: fblend_ring_uni_subtract,
        add_modulus: fblend_ring_uni_add,
        subtract_modulus: fblend_ring_uni_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

/// Naturally continuous, values wrap aroud the outer limits, interpolation
/// recommended, with a natural center point at 0.
/// Range: [-1.0,1.0]
/// Example: fully commutated pan or tilt
pub static continuous_ring_bipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_ring_bi_mean,
        add: fblend_ring_bi_add,
        subtract: fblend_ring_bi_subtract,
        add_modulus: fblend_ring_bi_add,
        subtract_modulus: fblend_ring_bi_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

/// Placeholder for integer blender methods that require more research before
/// we decide how to implement them. (For example, we're not sure how to
/// define multiply mode for discrete values.) Let's just clobber for now.
fn iblend_todo(a: i64, _b: i64) -> i64 {
    a
}

/// Naturally discontinuous, values wrap around the outer limits. Interpolation
/// is conceivably mechanically or logically meaningful, but it is aesthetically
/// discouraged.
/// Range: Int indexed from 0 TODO: or with a range?
/// Example: litho index
pub static discrete_ring: Topo = Topo {
    continuous: false,
    blend_encouraged: false,
    blend_meaningful: true,
    blenders: DiscreteBlenders(DiscreteBlenderTable {
        clobber: iblend_clobber,
        max: iblend_euclid_max,
        min: iblend_euclid_min,
        median: iblend_ring_median,
        add: iblend_ring_add,
        subtract: iblend_ring_subtract,
        add_modulus: iblend_ring_add,
        subtract_modulus: iblend_ring_subtract,
        multiply: iblend_todo,
        abs_max: iblend_euclid_max, // or _abs_max if we don't want to restrict range >= 0
        abs_min: iblend_euclid_min, // ditto
    })
};

/// Naturally discontinuous, values bounded. Interpolation is conceivably
/// mechanically or logically meaningful, but it is aesthetically discouraged.
/// Range: Int indexed from 0
/// Example: linear 35mm slide tray index
pub static discrete_array: Topo = Topo {
    continuous: false,
    blend_encouraged: false,
    blend_meaningful: true,
    blenders: DiscreteBlenders(DiscreteBlenderTable {
        clobber: iblend_clobber,
        max: iblend_euclid_max,
        min: iblend_euclid_min,
        median: iblend_euclid_median,
        add: iblend_euclid_add,
        subtract: iblend_euclid_subtract,
        add_modulus: iblend_euclid_add,
        subtract_modulus: iblend_euclid_subtract,
        multiply: iblend_todo,
        abs_max: iblend_euclid_max, // or _abs_max if we don't want to restrict range >= 0
        abs_min: iblend_euclid_min, // ditto
    })
};

/// Naturally discontinuous, values bounded. Interpolation is mechanically or
/// logically inconceivable and therefore aesthetically forbidden.
/// Nevertheless, interpolated values can be numerically computed, so we
/// implement them to make glitchers and experimenters happy -- and for
/// uniformity and completeness.
/// Range: Int indexed from 0
/// Example: color wheel mode
pub static discrete_set: Topo = Topo {
    continuous: false,
    blend_encouraged: false,
    blend_meaningful: false,
    blenders: DiscreteBlenders(DiscreteBlenderTable {
        // Same as discreteArray, e.g. for glitching between modes
        clobber: iblend_clobber,
        max: iblend_euclid_max,
        min: iblend_euclid_min,
        median: iblend_euclid_median,
        add: iblend_euclid_add,
        subtract: iblend_euclid_subtract,
        add_modulus: iblend_euclid_add,
        subtract_modulus: iblend_euclid_subtract,
        multiply: iblend_todo,
        abs_max: iblend_euclid_max, // or _abs_max if we don't want to restrict range >= 0
        abs_min: iblend_euclid_min, // ditto
    })
};
