//! The "discovered GUIDs" array: a separate appended region of the save file, not part of the
//! normal hash-table field storage, tracking specific in-world entity/event instances by 64-bit
//! GUID (individual bubbul enemies, sage-will pedestals, and a few other categories out of scope
//! here). Located one pointer dereference past the hash table's own sentinel slot
//! (`hash_table_end`): the 4 bytes stored there are the address of the array itself, mirroring
//! `SavegameEditor._getOffsets`'s `this.guidsArrayOffset=tempFile.readU32(i+4)` at the sentinel
//! row (the same row `totk::hashtable::find_hash_table_end` already scans for, just reading its
//! value instead of stopping at its position).

use std::collections::HashSet;

use crate::binary::SaveBuffer;
use crate::error::SaveError;

/// Reads every GUID out of the appended, zero-terminated `[u64_le]...` array, mirroring
/// `SavegameEditor.guidsArray`'s load loop (`lower`/`upper` u32 halves, low word first, stopping
/// at the first all-zero entry).
pub fn read_discovered_guids(buf: &SaveBuffer, hash_table_end: usize) -> Result<HashSet<u64>, SaveError> {
    let array_addr = buf.read_u32(hash_table_end)? as usize;
    let mut guids = HashSet::new();
    let mut offset = array_addr;
    while offset + 8 <= buf.len() {
        let value = buf.read_u64(offset)?;
        if value == 0 {
            break;
        }
        guids.insert(value);
        offset += 8;
    }
    Ok(guids)
}
