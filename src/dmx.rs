//! For modeling DMX attributes in DMX device profiles.
//! If you have many of the same device, they will all share the same profile.

use render::DmxAttributeRenderer;

use std::cell::RefCell;
use std::cell::RefMut;
use std::rc::Rc;

/// It is the responsibility of DeviceEndpoint.render() to interpret dmx_offset.
/// Normally this value specifies insertion order within the serialized output.
// REF: We might need to collapse this into the DmxAttributeRenderers.
pub enum DmxAddressOffset {
    DmxAddressOffsetSingle(uint), // Might not need anything else.

    // Not yet implemented, maybe not necessary: map to scattered, noncontiguous
    // values in the output buffer, as is often the case with CMYK moving
    // lights. (This might not be needed if we can achieve the same thing by
    // composing CMYK as a C attribute + an M attribute + a Y attribute + a
    // K attribute, each with its own offset.)
    // DmxAddressOffsetMultiple(~[uint]),
}

/// Specify than an attribute should be rendered with a specific function at
/// the given offset within the Device's slice of the DMX framebuffer.
// REF: consider consolidating with DmxAddressOffset.
pub struct DmxMap {
    /// For DMX, this is channel offset with the profile, e.g. pan @ ch3.
    pub offset: DmxAddressOffset,
    pub renderer: DmxAttributeRenderer,
}

/// Identify a logical DMX universe. This universe may or may not be mapped to
/// one or more output ports. Typically each universe is mapped to exactly one
/// port.
pub struct DmxUniverse {
    pub id: u32,
    pub name: ~str,
    pub frame: [u8, ..512], // TODO: REF to Vec<u8>, "no box" says CM
}

/// Situate a Device within a slice of a DmxUniverse. For example, say that a
/// particular RGB color changer occupies channels 1-3.
pub struct DmxAddr {
    pub universe: Rc<RefCell<DmxUniverse>>,
    pub address: uint, // TODO: statically constrain to 0..511 if possible in Rust
    pub length: uint, // the number of byte-sized channels occuped by this Profile in a universe
}

impl DmxAddr {

    pub fn try_get_univ_ref<'a>(&'a self) -> Option<RefMut<'a, DmxUniverse>> {
        self.universe.deref().try_borrow_mut()
    }

    /// Extract the writable slice of the DMX universe that belongs to a
    /// particular Device. This prevents a Device from erroneously overwriting
    /// portions of the framebuffer that do not belong to it. Note that we
    /// deliberately make no effort in this architectural layer to prevent users
    /// from defining overlapping Devices, because although it would be
    /// unconventional, it may prove necessary in some experimental contexts.
    /// A higher control layer should warn the user about conflicts/overlaps.

    pub fn slice_universe<'a>(&self, univ_ref: &'a mut RefMut<'a, DmxUniverse>) -> &'a mut [u8] {

        univ_ref.deref_mut().frame.mut_slice(self.address, self.address + self.length)
    }
}
