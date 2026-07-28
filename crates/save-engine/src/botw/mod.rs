pub mod strings;
pub mod versions;

use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;
use crate::hashtable::scan_offsets;

pub const MAX_ITEMS: usize = 420;

/// Hash -> field name table, ported from `zelda-botw.js`'s `Hashes` array. MUST stay
/// sorted in ascending hash order (see Global Constraints in the plan).
const HASHES: [(u32, &str); 33] = [
    (0x0bee9e46, "MAP"),
    (0x0cbf052a, "FLAGS_BOW"),
    (0x1e3fd294, "FLAGSV_BOW"),
    (0x23149bf8, "RUPEES"),
    (0x2906f327, "MAX_HEARTS"),
    (0x333aa6e5, "HORSE_SADDLES"),
    (0x3adff047, "MAX_STAMINA"),
    (0x441b7231, "DEFEATED_MOLDUGA_COUNTER"),
    (0x54679940, "DEFEATED_HINOX_COUNTER"),
    (0x57ee221d, "FLAGS_WEAPON"),
    (0x5f283289, "ITEMS"),
    (0x6150c6be, "HORSE_REINS"),
    (0x698266be, "DEFEATED_TALUS_COUNTER"),
    (0x69f17e8a, "FLAGSV_SHIELD"),
    (0x6a09fc59, "ITEMS_QUANTITY"),
    (0x73c29681, "PLAYTIME"),
    (0x7b74e117, "HORSE_NAMES"),
    (0x8a94e07a, "KOROK_SEED_COUNTER"),
    (0x9383490e, "MapApp_MapIconNo"),
    (0x97f925c3, "RELIC_GERUDO"),
    (0x982ba201, "HORSE_POSITION"),
    (0x9c6cfd3f, "HORSE_MANES"),
    (0xa40ba103, "PLAYER_POSITION"),
    (0xa6d926bc, "FLAGSV_WEAPON"),
    (0xc247b696, "HORSE_TYPES"),
    (0xc5238d2b, "FLAGS_SHIELD"),
    (0xc9328299, "MOTORCYCLE"),
    (0xce7afed3, "MONS"),
    (0xd913b769, "MAPTYPE"),
    (0xe1a0ca54, "HORSE_BONDS"),
    (0xea9def3f, "MapApp_MapIconPos"),
    (0xf1cf4807, "RELIC_GORON"),
    (0xfda0cde4, "RELIC_RITO"),
];

pub struct BotwSave {
    buf: SaveBuffer,
    offsets: HashMap<&'static str, usize>,
    pub version_index: usize,
    pub modded: bool,
}

impl BotwSave {
    pub fn load(bytes: Vec<u8>) -> Result<Self, SaveError> {
        let detected = versions::detect(&bytes).ok_or(SaveError::UnknownFormat)?;
        let mut buf = SaveBuffer::new(bytes);
        buf.little_endian = detected.little_endian;
        let offsets = scan_offsets(&buf, &HASHES);
        Ok(BotwSave {
            buf,
            offsets,
            version_index: detected.index,
            modded: detected.modded,
        })
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.buf.into_bytes()
    }

    fn offset(&self, name: &'static str) -> Result<usize, SaveError> {
        self.offsets.get(name).copied().ok_or(SaveError::MissingField(name))
    }

    // --- scalar stats ---

    pub fn rupees(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("RUPEES")?))
    }
    pub fn set_rupees(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RUPEES")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn mons(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("MONS")?))
    }
    pub fn set_mons(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MONS")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn max_hearts(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("MAX_HEARTS")?))
    }
    pub fn set_max_hearts(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_HEARTS")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn max_stamina(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("MAX_STAMINA")?))
    }
    pub fn set_max_stamina(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_STAMINA")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn relic_gerudo(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("RELIC_GERUDO")?))
    }
    pub fn set_relic_gerudo(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RELIC_GERUDO")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn relic_goron(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("RELIC_GORON")?))
    }
    pub fn set_relic_goron(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RELIC_GORON")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn relic_rito(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("RELIC_RITO")?))
    }
    pub fn set_relic_rito(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("RELIC_RITO")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn korok_seed_counter(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("KOROK_SEED_COUNTER")?))
    }
    pub fn set_korok_seed_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("KOROK_SEED_COUNTER")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn defeated_hinox_counter(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("DEFEATED_HINOX_COUNTER")?))
    }
    pub fn set_defeated_hinox_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("DEFEATED_HINOX_COUNTER")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn defeated_talus_counter(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("DEFEATED_TALUS_COUNTER")?))
    }
    pub fn set_defeated_talus_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("DEFEATED_TALUS_COUNTER")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    pub fn defeated_molduga_counter(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("DEFEATED_MOLDUGA_COUNTER")?))
    }
    pub fn set_defeated_molduga_counter(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("DEFEATED_MOLDUGA_COUNTER")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    /// Raw seconds. Formatting into `H:MM:SS` (`_timeToString` upstream) is a UI concern.
    pub fn playtime_seconds(&self) -> Result<u32, SaveError> {
        Ok(self.buf.read_u32(self.offset("PLAYTIME")?))
    }
    pub fn set_playtime_seconds(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("PLAYTIME")?;
        self.buf.write_u32(o, val);
        Ok(())
    }

    /// Not present in every save version, hence `Option` rather than `Result`.
    pub fn motorcycle(&self) -> Option<bool> {
        self.offsets.get("MOTORCYCLE").map(|&o| self.buf.read_u32(o) != 0)
    }
    pub fn set_motorcycle(&mut self, val: bool) {
        if let Some(&o) = self.offsets.get("MOTORCYCLE") {
            self.buf.write_u32(o, if val { 1 } else { 0 });
        }
    }

    // --- positions ---

    pub fn player_position(&self) -> Result<(f32, f32, f32), SaveError> {
        let o = self.offset("PLAYER_POSITION")?;
        Ok((self.buf.read_f32(o), self.buf.read_f32(o + 8), self.buf.read_f32(o + 16)))
    }
    pub fn set_player_position(&mut self, x: f32, y: f32, z: f32) -> Result<(), SaveError> {
        let o = self.offset("PLAYER_POSITION")?;
        self.buf.write_f32(o, x);
        self.buf.write_f32(o + 8, y);
        self.buf.write_f32(o + 16, z);
        Ok(())
    }

    pub fn horse_position(&self) -> Result<(f32, f32, f32), SaveError> {
        let o = self.offset("HORSE_POSITION")?;
        Ok((self.buf.read_f32(o), self.buf.read_f32(o + 8), self.buf.read_f32(o + 16)))
    }
    pub fn set_horse_position(&mut self, x: f32, y: f32, z: f32) -> Result<(), SaveError> {
        let o = self.offset("HORSE_POSITION")?;
        self.buf.write_f32(o, x);
        self.buf.write_f32(o + 8, y);
        self.buf.write_f32(o + 16, z);
        Ok(())
    }

    pub fn map(&self) -> Result<String, SaveError> {
        let o = self.offset("MAP")?;
        Ok(strings::read_padded_string(&self.buf, o, 8))
    }
    pub fn set_map(&mut self, value: &str) -> Result<(), SaveError> {
        let o = self.offset("MAP")?;
        strings::write_padded_string(&mut self.buf, o, value, 8);
        Ok(())
    }

    pub fn map_type(&self) -> Result<String, SaveError> {
        let o = self.offset("MAPTYPE")?;
        Ok(strings::read_padded_string(&self.buf, o, 8))
    }
    pub fn set_map_type(&mut self, value: &str) -> Result<(), SaveError> {
        let o = self.offset("MAPTYPE")?;
        strings::write_padded_string(&mut self.buf, o, value, 8);
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct BotwItem {
    pub name: String,
    pub quantity: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ItemKind {
    Weapon,
    Bow,
    Shield,
    Other,
}

/// Classifies an item name id exactly enough to reproduce the upstream modifier-slot
/// counting logic. Display categorization (armor/materials/food/other) is a UI concern
/// and is intentionally not reproduced here.
fn classify(name: &str) -> ItemKind {
    if name.starts_with("Weapon_Sword_") || name.starts_with("Weapon_Lsword_") || name.starts_with("Weapon_Spear_") {
        ItemKind::Weapon
    } else if name.starts_with("Weapon_Bow_")
        || matches!(name, "NormalArrow" | "FireArrow" | "IceArrow" | "ElectricArrow" | "BombArrow_A" | "AncientArrow")
    {
        ItemKind::Bow
    } else if name.starts_with("Weapon_Shield_") {
        ItemKind::Shield
    } else {
        ItemKind::Other
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ItemModifier {
    pub modifier: u32,
    pub value: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum ModifierCategory {
    Weapon,
    Bow,
    Shield,
}

impl BotwSave {
    /// Reads item slots until the first empty name (or `MAX_ITEMS`), mirroring `load()`'s
    /// item loop.
    pub fn items(&self) -> Result<Vec<BotwItem>, SaveError> {
        let items_offset = self.offset("ITEMS")?;
        let qty_offset = self.offset("ITEMS_QUANTITY")?;
        let mut result = Vec::new();
        for i in 0..MAX_ITEMS {
            let name = strings::read_string64(&self.buf, items_offset, i);
            if name.is_empty() {
                break;
            }
            let quantity = self.buf.read_u32(qty_offset + i * 8);
            result.push(BotwItem { name, quantity });
        }
        Ok(result)
    }

    pub fn set_item(&mut self, index: usize, name: &str, quantity: u32) -> Result<(), SaveError> {
        let items_offset = self.offset("ITEMS")?;
        let qty_offset = self.offset("ITEMS_QUANTITY")?;
        strings::write_string64(&mut self.buf, items_offset, index, name);
        self.buf.write_u32(qty_offset + index * 8, quantity);
        Ok(())
    }

    /// Returns (weapon, bow, shield) modifier lists. Slot counts are derived by walking
    /// the item list and tracking which contiguous category block is currently active —
    /// ported 1:1 from the upstream `search` state machine in `load()`.
    pub fn modifiers(&self) -> Result<(Vec<ItemModifier>, Vec<ItemModifier>, Vec<ItemModifier>), SaveError> {
        let items = self.items()?;
        let mut counts = [0usize; 3]; // weapon, bow, shield
        let mut search = 0u8; // 0: weapons, 1: bows, 2: shields, 3: done

        for item in &items {
            let kind = classify(&item.name);
            if search == 0 && kind == ItemKind::Bow {
                search = 1;
            } else if search == 0 && kind == ItemKind::Shield {
                search = 2;
            } else if search == 1 && kind == ItemKind::Shield {
                search = 2;
            } else if kind == ItemKind::Other {
                search = 3;
            }

            if kind == ItemKind::Weapon && search == 0 {
                counts[0] += 1;
            } else if kind == ItemKind::Bow && search == 1 && item.name.starts_with("Weapon_") {
                counts[1] += 1;
            } else if kind == ItemKind::Shield && search == 2 {
                counts[2] += 1;
            }
        }

        Ok((
            self.read_modifier_slots("FLAGS_WEAPON", "FLAGSV_WEAPON", counts[0])?,
            self.read_modifier_slots("FLAGS_BOW", "FLAGSV_BOW", counts[1])?,
            self.read_modifier_slots("FLAGS_SHIELD", "FLAGSV_SHIELD", counts[2])?,
        ))
    }

    fn read_modifier_slots(
        &self,
        flag_hash: &'static str,
        value_hash: &'static str,
        count: usize,
    ) -> Result<Vec<ItemModifier>, SaveError> {
        let flag_offset = self.offset(flag_hash)?;
        let value_offset = self.offset(value_hash)?;
        let mut out = Vec::with_capacity(count);
        for i in 0..count {
            out.push(ItemModifier {
                modifier: self.buf.read_u32(flag_offset + i * 8),
                value: self.buf.read_u32(value_offset + i * 8),
            });
        }
        Ok(out)
    }

    pub fn set_modifier(
        &mut self,
        category: ModifierCategory,
        index: usize,
        modifier: u32,
        value: u32,
    ) -> Result<(), SaveError> {
        let (flag_hash, value_hash) = match category {
            ModifierCategory::Weapon => ("FLAGS_WEAPON", "FLAGSV_WEAPON"),
            ModifierCategory::Bow => ("FLAGS_BOW", "FLAGSV_BOW"),
            ModifierCategory::Shield => ("FLAGS_SHIELD", "FLAGSV_SHIELD"),
        };
        let flag_offset = self.offset(flag_hash)?;
        let value_offset = self.offset(value_hash)?;
        self.buf.write_u32(flag_offset + index * 8, modifier);
        self.buf.write_u32(value_offset + index * 8, value);
        Ok(())
    }
}
