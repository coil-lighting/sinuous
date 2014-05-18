use blend::fblend_clobber;
use blend::fblend_euclid_max;
use blend::fblend_euclid_min;
use blend::fblend_euclid_median;
use blend::fblend_ring_uni_median;
use blend::fblend_ring_bi_median;
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
use blend::iblend_euclidAdd;
use blend::iblend_ring_subtract;
use blend::iblend_euclid_subtract;


type Blendf = fn(a:f64, b:f64) -> f64;

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

// TODO remember why I decided that range was essential with these, or simplify
// TODO wrap Blendi items to take (and discard) a range, for uniformity
type Blendi = fn(a:i64, b:i64) -> i64;
type BlendRangedi = fn(a:i64, b:i64, minimum: i64, maximum: i64) -> i64;

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

// A static lookup table of high-level blend modes to implementing functions
enum BlenderTable {
    // there must be a less awkward way to do this...
    ContinuousBlenders(ContinuousBlenderTable),
    DiscreteBlenders(DiscreteBlenderTable),
}

// Topology descriptors constrain the parametric range for all attribute values.
pub struct Topo {
    // Quasi-continuous (encoded as f64) or discrete (encoded as i64)
    continuous: bool,

    // Interpolation aesthetically encouraged
    blend_encouraged: bool,

    // Interpolation logically or mechanically meaningful
    blend_meaningful: bool,
    blenders: BlenderTable,
}

// Naturally continuous, values bounded, interpolation recommended.
// Range: [0.0,1.0]
// Example: dimmer
pub static continuous_euclidian_unipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_euclid_median,
        add: fblend_euclid_uni_add,
        subtract: fblend_euclid_uni_subtract,
        add_modulus: fblend_ring_uni_add,
        subtract_modulus: fblend_ring_uni_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

// // Naturally continuous, values bounded, interpolation recommended.
// // Range: [-1.0,1.0]
// // Example: X- or Y-position on a bounded pivot or linear track
pub static continuous_euclidian_bipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_euclid_median,
        add: fblend_euclid_bi_add,
        subtract: fblend_euclid_bi_subtract,
        add_modulus: fblend_ring_bi_add,
        subtract_modulus: fblend_ring_bi_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

// // Naturally continuous, values wrap, interpolation recommended.
// // Range: [0.0,1.0]
// // Example: angle of rotation
pub static continuous_ring_unipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_ring_uni_median,
        add: fblend_ring_uni_add,
        subtract: fblend_ring_uni_subtract,
        add_modulus: fblend_ring_uni_add,
        subtract_modulus: fblend_ring_uni_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

// // Naturally continuous, values wrap, interpolation recommended, with a
// // natural center point at 0.
// // Range: [-1.0,1.0]
// // Example: fully commutated pan or tilt
pub static continuous_ring_bipolar: Topo = Topo {
    continuous: true,
    blend_encouraged: true,
    blend_meaningful: true,
    blenders: ContinuousBlenders(ContinuousBlenderTable {
        clobber: fblend_clobber,
        max: fblend_euclid_max,
        min: fblend_euclid_min,
        median: fblend_ring_bi_median,
        add: fblend_ring_bi_add,
        subtract: fblend_ring_bi_subtract,
        add_modulus: fblend_ring_bi_add,
        subtract_modulus: fblend_ring_bi_subtract,
        multiply: fblend_euclid_multiply,
        abs_max: fblend_euclid_max,
        abs_min: fblend_euclid_min,
    })
};

fn iblend_TODO(a: i64, _b: i64) -> i64 {
    // Not sure how to define multiply mode for discrete values. Clobber for now.
    a
}

// Naturally discontinuous, values wrap, interpolation conceivably
// mechanically/logically meaningful, but aesthetically discouraged.
// Range: Int indexed from 0
// Example: litho index
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
        multiply: iblend_TODO,
        abs_max: iblend_euclid_max, // or _abs_max if we don't want to restrict range >= 0
        abs_min: iblend_euclid_min, // ditto
    })
};

// Naturally discontinuous, values bounded, interpolation conceivably
// mechanically/logically meaningful, but aesthetically discouraged.
// Range: Int indexed from 0
// Example: linear 35mm slide tray index
pub static discrete_array: Topo = Topo {
    continuous: false,
    blend_encouraged: false,
    blend_meaningful: true,
    blenders: DiscreteBlenders(DiscreteBlenderTable {
        clobber: iblend_clobber,
        max: iblend_euclid_max,
        min: iblend_euclid_min,
        median: iblend_euclid_median,
        add: iblend_euclidAdd,
        subtract: iblend_euclid_subtract,
        add_modulus: iblend_euclidAdd,
        subtract_modulus: iblend_euclid_subtract,
        multiply: iblend_TODO,
        abs_max: iblend_euclid_max, // or _abs_max if we don't want to restrict range >= 0
        abs_min: iblend_euclid_min, // ditto
    })
};

// Naturally discontinuous, values bounded, interpolation
// mechanically/logically inconceivable and therefore forbidden.
// Nevertheless, they can be numerically computed, so we implement them to make
// experimenters happy -- and for uniformity.
// Range: Int indexed from 0
// Example: color wheel mode
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
        add: iblend_euclidAdd,
        subtract: iblend_euclid_subtract,
        add_modulus: iblend_euclidAdd,
        subtract_modulus: iblend_euclid_subtract,
        multiply: iblend_TODO,
        abs_max: iblend_euclid_max, // or _abs_max if we don't want to restrict range >= 0
        abs_min: iblend_euclid_min, // ditto
    })
};
