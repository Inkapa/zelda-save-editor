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
