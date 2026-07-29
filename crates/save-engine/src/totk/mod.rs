pub mod hashtable;
pub mod murmur3;
pub mod pouch;
pub mod strings;
pub mod versions;

use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;

/// Hash -> (field name, is_pointer) table, ported from `zelda-totk.js`'s `Hashes` array,
/// restricted to the 11 hashes this crate exposes (see the design spec's non-goals for
/// what's deferred: MapData icons, AutoBuilder, and everything else in the source's
/// 18-entry table). Unlike BOTW's `HASHES`, this table has no ordering requirement —
/// `hashtable::scan_offsets` looks each hash up in a map, not via a moving cursor.
const HASHES: [(u32, &str, bool); 11] = [
    (0xfbe01da1, "MAX_LIFE", false),
    (0xa77921d7, "CURRENT_RUPEES", false),
    (0xf9212c74, "MAX_STAMINA", false),
    (0x15ec5858, "HORSE_INN_MEMBER_POINT", false),
    (0xe573f564, "PLAYTIME", false),
    (0xafd01d68, "MAX_ENERGY", false),
    (0xc884818d, "SAVE_POS", true),
    (0x1d6189da, "SEQUENCE_CURRENT_BANC", true),
    (0xd7a3f6ba, "POUCH_WEAPON_VALID_NUM", true),
    (0xc61785c2, "POUCH_BOW_VALID_NUM", true),
    (0x05271e7d, "POUCH_SHIELD_VALID_NUM", true),
];

pub struct TotkSave {
    buf: SaveBuffer,
    offsets: HashMap<&'static str, usize>,
    hash_table_end: usize,
    pub version_label: &'static str,
    pub modded: bool,
}

impl TotkSave {
    pub fn load(bytes: Vec<u8>) -> Result<Self, SaveError> {
        versions::check_magic_and_size(&bytes)?;
        let (version_label, modded) = versions::label(&bytes);
        let mut buf = SaveBuffer::new(bytes);
        buf.little_endian = true;
        let hash_table_end = hashtable::find_hash_table_end(&buf)?;
        let offsets = hashtable::scan_offsets(&buf, hash_table_end, &HASHES)?;
        Ok(TotkSave {
            buf,
            offsets,
            hash_table_end,
            version_label,
            modded,
        })
    }

    pub fn to_bytes(self) -> Vec<u8> {
        self.buf.into_bytes()
    }

    fn offset(&self, name: &'static str) -> Result<usize, SaveError> {
        self.offsets.get(name).copied().ok_or(SaveError::MissingField(name))
    }

    pub fn max_life(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_LIFE")?)
    }
    pub fn set_max_life(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_LIFE")?;
        self.buf.write_u32(o, val)
    }

    pub fn current_rupees(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("CURRENT_RUPEES")?)
    }
    pub fn set_current_rupees(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("CURRENT_RUPEES")?;
        self.buf.write_u32(o, val)
    }

    pub fn max_stamina(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_STAMINA")?)
    }
    pub fn set_max_stamina(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_STAMINA")?;
        self.buf.write_u32(o, val)
    }

    pub fn max_energy(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("MAX_ENERGY")?)
    }
    pub fn set_max_energy(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("MAX_ENERGY")?;
        self.buf.write_u32(o, val)
    }

    /// The source itself marks this hash's meaning as unconfirmed ("unknown key" in its
    /// own comment) — exposed under its source name rather than asserting false
    /// confidence about what it represents.
    pub fn playtime(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("PLAYTIME")?)
    }
    pub fn set_playtime(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("PLAYTIME")?;
        self.buf.write_u32(o, val)
    }

    pub fn horse_inn_member_point(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("HORSE_INN_MEMBER_POINT")?)
    }
    pub fn set_horse_inn_member_point(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("HORSE_INN_MEMBER_POINT")?;
        self.buf.write_u32(o, val)
    }

    pub fn save_pos(&self) -> Result<(f32, f32, f32), SaveError> {
        let o = self.offset("SAVE_POS")?;
        Ok((self.buf.read_f32(o)?, self.buf.read_f32(o + 4)?, self.buf.read_f32(o + 8)?))
    }
    pub fn set_save_pos(&mut self, x: f32, y: f32, z: f32) -> Result<(), SaveError> {
        let o = self.offset("SAVE_POS")?;
        self.buf.write_f32(o, x)?;
        self.buf.write_f32(o + 4, y)?;
        self.buf.write_f32(o + 8, z)?;
        Ok(())
    }

    pub fn sequence_current_banc(&self) -> Result<String, SaveError> {
        let o = self.offset("SEQUENCE_CURRENT_BANC")?;
        Ok(strings::read_string64(&self.buf, o))
    }
    pub fn set_sequence_current_banc(&mut self, value: &str) -> Result<(), SaveError> {
        let o = self.offset("SEQUENCE_CURRENT_BANC")?;
        strings::write_string64(&mut self.buf, o, value)
    }

    pub fn pouch_weapon_valid_num(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("POUCH_WEAPON_VALID_NUM")?)
    }
    pub fn set_pouch_weapon_valid_num(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("POUCH_WEAPON_VALID_NUM")?;
        self.buf.write_u32(o, val)
    }

    pub fn pouch_bow_valid_num(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("POUCH_BOW_VALID_NUM")?)
    }
    pub fn set_pouch_bow_valid_num(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("POUCH_BOW_VALID_NUM")?;
        self.buf.write_u32(o, val)
    }

    pub fn pouch_shield_valid_num(&self) -> Result<u32, SaveError> {
        self.buf.read_u32(self.offset("POUCH_SHIELD_VALID_NUM")?)
    }
    pub fn set_pouch_shield_valid_num(&mut self, val: u32) -> Result<(), SaveError> {
        let o = self.offset("POUCH_SHIELD_VALID_NUM")?;
        self.buf.write_u32(o, val)
    }

    pub fn pouch_weapons(&self) -> Result<Vec<pouch::WeaponEntry>, SaveError> {
        pouch::read_weapons(&self.buf, self.hash_table_end, self.pouch_weapon_valid_num()?)
    }
    pub fn set_pouch_weapons(&mut self, entries: &[pouch::WeaponEntry]) -> Result<(), SaveError> {
        pouch::write_weapons(&mut self.buf, self.hash_table_end, entries)?;
        self.set_pouch_weapon_valid_num(entries.len() as u32)
    }

    pub fn pouch_bows(&self) -> Result<Vec<pouch::BowEntry>, SaveError> {
        pouch::read_bows(&self.buf, self.hash_table_end, self.pouch_bow_valid_num()?)
    }
    pub fn set_pouch_bows(&mut self, entries: &[pouch::BowEntry]) -> Result<(), SaveError> {
        pouch::write_bows(&mut self.buf, self.hash_table_end, entries)?;
        self.set_pouch_bow_valid_num(entries.len() as u32)
    }

    pub fn pouch_shields(&self) -> Result<Vec<pouch::ShieldEntry>, SaveError> {
        pouch::read_shields(&self.buf, self.hash_table_end, self.pouch_shield_valid_num()?)
    }
    pub fn set_pouch_shields(&mut self, entries: &[pouch::ShieldEntry]) -> Result<(), SaveError> {
        pouch::write_shields(&mut self.buf, self.hash_table_end, entries)?;
        self.set_pouch_shield_valid_num(entries.len() as u32)
    }

    pub fn armor(&self) -> Result<Vec<pouch::ArmorEntry>, SaveError> {
        pouch::read_armor(&self.buf, self.hash_table_end)
    }
    pub fn set_armor(&mut self, entries: &[pouch::ArmorEntry]) -> Result<(), SaveError> {
        pouch::write_armor(&mut self.buf, self.hash_table_end, entries)
    }
}
