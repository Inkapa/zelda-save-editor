//! Player-placed map pins ("stamps"): the in-game "add pin"/"clear pins" feature
//! (`MapData.IconData.StampData.*`). Distinct from the advanced Master-mode "Map pins editor",
//! which also covers fast-travel markers and teleporters through the source's generic
//! Variable/Struct engine, out of scope here (that engine itself is a separate deferred slice).
//! Fixed 300-slot struct-of-arrays array (`MapPin.MAX` in the source), same
//! `[u32 capacity][element_0][element_1]...` layout as pouch/horse, addressed by three literal
//! hash constants already known from the source's own `Hashes` table (not murmur3-computed at
//! call time, like AutoBuild's core fields).
//!
//! `icon`/`layer` are raw hashes, same convention as every other `Enum`-typed field in this crate
//! (`pouch::WeaponEntry::modifier`, `pouch::ArmorEntry::dye_color`, ...): not decoded into a Rust
//! enum here, just exposed as-is.

use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::hashtable::scan_offsets;

/// `MapPin.MAX` in the source: the fixed number of stamp slots.
pub const CAPACITY: usize = 300;

/// `MapPin.ICON_NONE`: a slot with this icon hash is unused.
pub const ICON_NONE: u32 = 0x7e3d1e46;

pub struct MapPinEntry {
    /// `MapData.IconData.StampData.Type`, ported from `MapPin.ICON_*`.
    pub icon: u32,
    pub x: f32,
    pub y: f32,
    /// `MapData.IconData.StampData.Layer`, ported from `MapPin.MAP_*`.
    pub layer: u32,
}

impl MapPinEntry {
    pub fn is_free(&self) -> bool {
        self.icon == ICON_NONE
    }
}

fn resolve(buf: &SaveBuffer, hash_table_end: usize) -> Result<HashMap<&'static str, usize>, SaveError> {
    scan_offsets(
        buf,
        hash_table_end,
        &[
            (0x14d7f4c4, "TYPE", true),  // MapData.IconData.StampData.Type
            (0xf24fc2e7, "POS", true),   // MapData.IconData.StampData.Pos
            (0xd2025694, "LAYER", true), // MapData.IconData.StampData.Layer
        ],
    )
}

fn read_vector2f_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<(f32, f32), SaveError> {
    let base = array_addr + 4 + index * 0x08;
    Ok((buf.read_f32(base)?, buf.read_f32(base + 4)?))
}
fn write_vector2f_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: (f32, f32),
) -> Result<(), SaveError> {
    let base = array_addr + 4 + index * 0x08;
    buf.write_f32(base, value.0)?;
    buf.write_f32(base + 4, value.1)
}

pub fn read_map_pins(buf: &SaveBuffer, hash_table_end: usize) -> Result<Vec<MapPinEntry>, SaveError> {
    let offsets = resolve(buf, hash_table_end)?;
    let type_addr = offsets["TYPE"];
    let pos_addr = offsets["POS"];
    let layer_addr = offsets["LAYER"];
    let capacity = buf.read_u32(type_addr)? as usize;

    let mut entries = Vec::with_capacity(capacity);
    for i in 0..capacity {
        let icon = buf.read_u32(type_addr + 4 + i * 4)?;
        let (x, y) = read_vector2f_elem(buf, pos_addr, i)?;
        let layer = buf.read_u32(layer_addr + 4 + i * 4)?;
        entries.push(MapPinEntry { icon, x, y, layer });
    }
    Ok(entries)
}

/// Writes back exactly `capacity` (300) entries, same "always full width, no trimming"
/// convention as AutoBuild's Draft slots: a free pin is a slot whose icon is `ICON_NONE`, not an
/// omitted array entry.
pub fn write_map_pins(buf: &mut SaveBuffer, hash_table_end: usize, entries: &[MapPinEntry]) -> Result<(), SaveError> {
    let offsets = resolve(buf, hash_table_end)?;
    let type_addr = offsets["TYPE"];
    let pos_addr = offsets["POS"];
    let layer_addr = offsets["LAYER"];
    let capacity = buf.read_u32(type_addr)? as usize;

    if entries.len() != capacity {
        return Err(SaveError::SizeMismatch { expected: capacity, actual: entries.len() });
    }

    for (i, entry) in entries.iter().enumerate() {
        buf.write_u32(type_addr + 4 + i * 4, entry.icon)?;
        write_vector2f_elem(buf, pos_addr, i, (entry.x, entry.y))?;
        buf.write_u32(layer_addr + 4 + i * 4, entry.layer)?;
    }
    Ok(())
}
