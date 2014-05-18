//! For modeling DMX attributes in DMX device profiles.
use render::DmxAttributeRenderer;

// # dmx_offset - Specifies insertion order within the serialized output.
// # For DMX, this specifies byte offset (AKA channel offset) within the
// # fixture's data array. Offset may be an int or a sequence of ints.
// # If dmx_offset is an int, then this Attribute's rendered data is packed
// # sequentially starting at the given offset. If dmx_offset is an int
// # sequence, then this Attribute's rendered data occupies multiple channels
// # which are not necessarily contiguous. Each rendered datum is rendered at
// # its corresponding offset. For example, datum[2] would be rendered at
// # offset[2]. This typically happens in a CMYK profile where K is not
// # adjacent to C+M+Y. Only certain attribute types require mapped-
// # multichannel (sequential) dmx_offsets; see also
// # DMXAttributeRenderers.attributeTypes.

// It is the responsibility of DeviceEndpoint.render() to interpret dmx_offset.
// We might need to collapse this into the DmxAttributeRenderers.
pub enum DmxAddressOffset {
    DmxAddressOffsetSingle(uint),

    // DmxAddressOffsetMultiple(~[uint]), // Not yet implemented

    // TODO Is it really necessary to use a hashmap here? I sure as hell hope not.
    // DmxAddressOffsetMap(HashMap<~str, uint>), // Not yet implemented
}

pub struct DmxMap {
    pub offset: DmxAddressOffset, // channel offset with the profile, e.g. pan @ ch3
    pub renderer: DmxAttributeRenderer,
}

pub struct DmxUniverse {
    // Just a sketch...
    pub id: u32, // TEMP
    pub name: ~str,
    pub frame: ~[u8], // definitely not its final resting place
}

pub struct DmxAddr {
    // FUTURE: consider what it would take to render safely in parallel
    universe: Box<DmxUniverse>,
    address: uint, // TODO: statically constrain to 0..511 if possible
}
