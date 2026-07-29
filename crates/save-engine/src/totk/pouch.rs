//! Pouch item contents: weapons, bows, shields, armor, arrows, materials, food, key items, and
//! Zonai devices (save-format prefix `Pouch.SpecialParts.*` — see design spec). Every property
//! is stored as a separate struct-of-arrays blob: `[u32 capacity][element_0][element_1]...`,
//! resolved through the same pointer-hash mechanism as `SAVE_POS`/`SEQUENCE_CURRENT_BANC`
//! (`totk::hashtable::scan_offsets`, unchanged) — the hash itself is computed via
//! `totk::murmur3::hash32` from the field's name string, since (unlike the core slice's
//! hashes) these aren't literal constants in the source. See the design spec's Background and
//! "Field tables" section for every hash constant used below and how each was cross-checked.

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::totk::strings;
use crate::totk::murmur3::hash32;

/// Reads the `u32` item-capacity prefix at a resolved struct-of-arrays address.
fn array_capacity(buf: &SaveBuffer, array_addr: usize) -> Result<u32, SaveError> {
    buf.read_u32(array_addr)
}

/// Offset of element `index` in a fixed-`stride`-byte struct-of-arrays blob at `array_addr`.
fn element_offset(array_addr: usize, index: usize, stride: usize) -> usize {
    array_addr + 4 + index * stride
}

const STRIDE_STRING64: usize = 0x40;
const STRIDE_I32: usize = 4;

fn read_string64_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> String {
    strings::read_string64(buf, element_offset(array_addr, index, STRIDE_STRING64))
}
fn write_string64_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: &str,
) -> Result<(), SaveError> {
    strings::write_string64(buf, element_offset(array_addr, index, STRIDE_STRING64), value)
}
fn read_i32_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<i32, SaveError> {
    buf.read_i32(element_offset(array_addr, index, STRIDE_I32))
}
fn write_i32_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: i32,
) -> Result<(), SaveError> {
    buf.write_i32(element_offset(array_addr, index, STRIDE_I32), value)
}
/// `Enum`-typed fields (modifier, dye color, ...) are raw u32 hashes — same stride as i32,
/// unsigned read/write, no interpretation (see design spec non-goals).
fn read_u32_elem(buf: &SaveBuffer, array_addr: usize, index: usize) -> Result<u32, SaveError> {
    buf.read_u32(element_offset(array_addr, index, STRIDE_I32))
}
fn write_u32_elem(
    buf: &mut SaveBuffer,
    array_addr: usize,
    index: usize,
    value: u32,
) -> Result<(), SaveError> {
    buf.write_u32(element_offset(array_addr, index, STRIDE_I32), value)
}

/// Blanks (empties) the id field for `index`, marking a pouch slot unused — mirrors
/// `Pouch.prototype.save`'s "remove empty items" loop. Used by every category's `set_*`.
fn clear_id_elem(buf: &mut SaveBuffer, array_addr: usize, index: usize) -> Result<(), SaveError> {
    write_string64_elem(buf, array_addr, index, "")
}

/// Builds the `(hash, key, is_pointer=true)` list `scan_offsets` expects from a
/// `(field_name, key)` table, hashing each field name at call time via `murmur3::hash32`.
fn resolve_category(
    buf: &SaveBuffer,
    hash_table_end: usize,
    field_names: &[(&'static str, &'static str)],
) -> Result<std::collections::HashMap<&'static str, usize>, SaveError> {
    let hashes: Vec<(u32, &'static str, bool)> = field_names
        .iter()
        .map(|&(name, key)| (hash32(name), key, true))
        .collect();
    crate::totk::hashtable::scan_offsets(buf, hash_table_end, &hashes)
}

// ---------------------------------------------------------------------------------------------
// Weapons
// ---------------------------------------------------------------------------------------------

pub struct WeaponEntry {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
    pub fuse_id: String,
    pub fuse_durability: i32,
    pub extra_durability: i32,
    pub record_extra_durability: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.WEAPONS` —
/// hashed at call time via `murmur3::hash32` rather than stored as literal constants (see
/// design spec Background: these are computed hashes in the source, unlike the core slice's).
fn weapon_field_names() -> [(&'static str, &'static str); 8] {
    [
        ("Pouch.Weapon.Content.Name", "NAME"),
        ("Pouch.Weapon.Content.Life", "LIFE"),
        ("Pouch.Weapon.Content.Effect.Type", "MODIFIER"),
        ("Pouch.Weapon.Content.Effect.Value", "MODIFIER_VALUE"),
        ("Pouch.Weapon.Content.Combined.Name", "FUSE_ID"),
        ("Pouch.Weapon.Content.Combined.Life", "FUSE_DURABILITY"),
        ("Pouch.Weapon.Content.ExtraLife", "EXTRA_DURABILITY"),
        ("Pouch.Weapon.Content.RecordExtraLife", "RECORD_EXTRA_DURABILITY"),
    ]
}

pub fn read_weapons(
    buf: &SaveBuffer,
    hash_table_end: usize,
    valid_num: u32,
) -> Result<Vec<WeaponEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &weapon_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];
    let record_extra_durability_addr = offsets["RECORD_EXTRA_DURABILITY"];

    let mut entries = Vec::new();
    for i in 0..(valid_num as usize) {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(WeaponEntry {
            id,
            durability: read_i32_elem(buf, life_addr, i)?,
            modifier: read_u32_elem(buf, modifier_addr, i)?,
            modifier_value: read_i32_elem(buf, modifier_value_addr, i)?,
            fuse_id: read_string64_elem(buf, fuse_id_addr, i),
            fuse_durability: read_i32_elem(buf, fuse_durability_addr, i)?,
            extra_durability: read_i32_elem(buf, extra_durability_addr, i)?,
            record_extra_durability: read_i32_elem(buf, record_extra_durability_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_weapons(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[WeaponEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &weapon_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];
    let record_extra_durability_addr = offsets["RECORD_EXTRA_DURABILITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, life_addr, i, entry.durability)?;
        write_u32_elem(buf, modifier_addr, i, entry.modifier)?;
        write_i32_elem(buf, modifier_value_addr, i, entry.modifier_value)?;
        write_string64_elem(buf, fuse_id_addr, i, &entry.fuse_id)?;
        write_i32_elem(buf, fuse_durability_addr, i, entry.fuse_durability)?;
        write_i32_elem(buf, extra_durability_addr, i, entry.extra_durability)?;
        write_i32_elem(buf, record_extra_durability_addr, i, entry.record_extra_durability)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Bows
// ---------------------------------------------------------------------------------------------

pub struct BowEntry {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.BOWS` —
/// bows have no fuse fields (`Equipment.prototype.isFusable` excludes bows in the source).
fn bow_field_names() -> [(&'static str, &'static str); 4] {
    [
        ("Pouch.Bow.Content.Name", "NAME"),
        ("Pouch.Bow.Content.Life", "LIFE"),
        ("Pouch.Bow.Content.Effect.Type", "MODIFIER"),
        ("Pouch.Bow.Content.Effect.Value", "MODIFIER_VALUE"),
    ]
}

pub fn read_bows(
    buf: &SaveBuffer,
    hash_table_end: usize,
    valid_num: u32,
) -> Result<Vec<BowEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &bow_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];

    let mut entries = Vec::new();
    for i in 0..(valid_num as usize) {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(BowEntry {
            id,
            durability: read_i32_elem(buf, life_addr, i)?,
            modifier: read_u32_elem(buf, modifier_addr, i)?,
            modifier_value: read_i32_elem(buf, modifier_value_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_bows(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[BowEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &bow_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, life_addr, i, entry.durability)?;
        write_u32_elem(buf, modifier_addr, i, entry.modifier)?;
        write_i32_elem(buf, modifier_value_addr, i, entry.modifier_value)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Shields
// ---------------------------------------------------------------------------------------------

pub struct ShieldEntry {
    pub id: String,
    pub durability: i32,
    pub modifier: u32,
    pub modifier_value: i32,
    pub fuse_id: String,
    pub fuse_durability: i32,
    pub extra_durability: i32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.SHIELDS` —
/// shields are fusable (unlike bows) but have no `RecordExtraLife` field (unlike weapons): 7
/// entries vs weapons' 8.
fn shield_field_names() -> [(&'static str, &'static str); 7] {
    [
        ("Pouch.Shield.Content.Name", "NAME"),
        ("Pouch.Shield.Content.Life", "LIFE"),
        ("Pouch.Shield.Content.Effect.Type", "MODIFIER"),
        ("Pouch.Shield.Content.Effect.Value", "MODIFIER_VALUE"),
        ("Pouch.Shield.Content.Combined.Name", "FUSE_ID"),
        ("Pouch.Shield.Content.Combined.Life", "FUSE_DURABILITY"),
        ("Pouch.Shield.Content.ExtraLife", "EXTRA_DURABILITY"),
    ]
}

pub fn read_shields(
    buf: &SaveBuffer,
    hash_table_end: usize,
    valid_num: u32,
) -> Result<Vec<ShieldEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &shield_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];

    let mut entries = Vec::new();
    for i in 0..(valid_num as usize) {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(ShieldEntry {
            id,
            durability: read_i32_elem(buf, life_addr, i)?,
            modifier: read_u32_elem(buf, modifier_addr, i)?,
            modifier_value: read_i32_elem(buf, modifier_value_addr, i)?,
            fuse_id: read_string64_elem(buf, fuse_id_addr, i),
            fuse_durability: read_i32_elem(buf, fuse_durability_addr, i)?,
            extra_durability: read_i32_elem(buf, extra_durability_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_shields(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[ShieldEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &shield_field_names())?;
    let name_addr = offsets["NAME"];
    let life_addr = offsets["LIFE"];
    let modifier_addr = offsets["MODIFIER"];
    let modifier_value_addr = offsets["MODIFIER_VALUE"];
    let fuse_id_addr = offsets["FUSE_ID"];
    let fuse_durability_addr = offsets["FUSE_DURABILITY"];
    let extra_durability_addr = offsets["EXTRA_DURABILITY"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_i32_elem(buf, life_addr, i, entry.durability)?;
        write_u32_elem(buf, modifier_addr, i, entry.modifier)?;
        write_i32_elem(buf, modifier_value_addr, i, entry.modifier_value)?;
        write_string64_elem(buf, fuse_id_addr, i, &entry.fuse_id)?;
        write_i32_elem(buf, fuse_durability_addr, i, entry.fuse_durability)?;
        write_i32_elem(buf, extra_durability_addr, i, entry.extra_durability)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------
// Armor
// ---------------------------------------------------------------------------------------------

pub struct ArmorEntry {
    pub id: String,
    pub dye_color: u32,
}

/// Field names exactly as they appear in `zelda-totk.class.pouch.js`'s `Pouch.Structs.ARMOR` —
/// armor has no valid_num entry in the core slice's HASHES table; instead, we scan until empty id.
fn armor_field_names() -> [(&'static str, &'static str); 2] {
    [
        ("Pouch.Armor.Content.Name", "NAME"),
        ("Pouch.Armor.Content.ColorVariation", "DYE_COLOR"),
    ]
}

pub fn read_armor(
    buf: &SaveBuffer,
    hash_table_end: usize,
) -> Result<Vec<ArmorEntry>, SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &armor_field_names())?;
    let name_addr = offsets["NAME"];
    let dye_color_addr = offsets["DYE_COLOR"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    let mut entries = Vec::new();
    for i in 0..capacity {
        let id = read_string64_elem(buf, name_addr, i);
        if id.is_empty() {
            break;
        }
        entries.push(ArmorEntry {
            id,
            dye_color: read_u32_elem(buf, dye_color_addr, i)?,
        });
    }
    Ok(entries)
}

pub fn write_armor(
    buf: &mut SaveBuffer,
    hash_table_end: usize,
    entries: &[ArmorEntry],
) -> Result<(), SaveError> {
    let offsets = resolve_category(buf, hash_table_end, &armor_field_names())?;
    let name_addr = offsets["NAME"];
    let dye_color_addr = offsets["DYE_COLOR"];
    let capacity = array_capacity(buf, name_addr)? as usize;

    for (i, entry) in entries.iter().enumerate() {
        write_string64_elem(buf, name_addr, i, &entry.id)?;
        write_u32_elem(buf, dye_color_addr, i, entry.dye_color)?;
    }
    for i in entries.len()..capacity {
        clear_id_elem(buf, name_addr, i)?;
    }
    Ok(())
}
