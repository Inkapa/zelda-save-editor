pub mod hashtable;
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
}
