use std::collections::HashMap;

use crate::binary::SaveBuffer;
use crate::error::SaveError;

const SENTINEL_HASH: u32 = 0xa3db7114; // "MetaData.SaveTypeHash" in the source
const HASH_TABLE_START: usize = 0x28;

/// Finds the end of the simple hash-value region by scanning for the sentinel hash,
/// which marks where the "guids array" (discovered-locations tracking, out of scope for
/// this crate) begins. Returns the offset of the 4 bytes immediately after the sentinel
/// hash, mirroring `Variable.findHashTableEnd`.
pub fn find_hash_table_end(buf: &SaveBuffer) -> Result<usize, SaveError> {
    let mut i = HASH_TABLE_START;
    while i + 8 <= buf.len() {
        if buf.read_u32(i)? == SENTINEL_HASH {
            return Ok(i + 4);
        }
        i += 8;
    }
    Err(SaveError::UnknownFormat)
}

/// Scans `0x28..hash_table_end` for each `(hash, name, is_pointer)` triple in `hashes`.
/// For direct hashes, records the offset immediately after the hash. For pointer
/// hashes, dereferences once more: the 4 bytes after the hash are themselves an address
/// to the actual data. Returns an error naming the first hash not found, mirroring
/// `_getOffsets`'s `foundAllHashes` gate: every hash in `hashes` must be present for a
/// save to be considered valid.
pub fn scan_offsets(
    buf: &SaveBuffer,
    hash_table_end: usize,
    hashes: &[(u32, &'static str, bool)],
) -> Result<HashMap<&'static str, usize>, SaveError> {
    let lookup: HashMap<u32, (&'static str, bool)> = hashes
        .iter()
        .map(|&(hash, name, is_pointer)| (hash, (name, is_pointer)))
        .collect();

    let mut offsets = HashMap::new();
    let mut i = HASH_TABLE_START;
    while i + 8 <= hash_table_end {
        let hash = buf.read_u32(i)?;
        if let Some(&(name, is_pointer)) = lookup.get(&hash) {
            let offset = if is_pointer {
                buf.read_u32(i + 4)? as usize
            } else {
                i + 4
            };
            offsets.insert(name, offset);
        }
        i += 8;
    }

    for &(_, name, _) in hashes {
        if !offsets.contains_key(name) {
            return Err(SaveError::MissingField(name));
        }
    }

    Ok(offsets)
}

/// Resolves the value-offset for every hash in `hashes` that's actually present, scanning
/// once from `0x28` to `hash_table_end`, mirroring `_getOffsetsByHashes`, which (unlike
/// `_getOffsets`/`scan_offsets`) never dereferences a pointer and silently omits any hash not
/// found rather than erroring. Used for completionism flags (shrines/koroks/bosses/locations),
/// which aren't part of the small required core field set.
pub fn resolve_present_hashes(
    buf: &SaveBuffer,
    hash_table_end: usize,
    hashes: &[u32],
) -> Result<HashMap<u32, usize>, SaveError> {
    let target: std::collections::HashSet<u32> = hashes.iter().copied().collect();
    let mut offsets = HashMap::new();
    let mut i = HASH_TABLE_START;
    while i + 8 <= hash_table_end {
        let hash = buf.read_u32(i)?;
        if target.contains(&hash) {
            offsets.insert(hash, i + 4);
        }
        i += 8;
    }
    Ok(offsets)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer_with_slots(slots: &[(u32, u32)]) -> SaveBuffer {
        let mut data = vec![0u8; HASH_TABLE_START];
        for (hash, value) in slots {
            data.extend_from_slice(&hash.to_le_bytes());
            data.extend_from_slice(&value.to_le_bytes());
        }
        let mut buf = SaveBuffer::new(data);
        buf.little_endian = true;
        buf
    }

    #[test]
    fn resolves_direct_and_pointer_hashes() {
        // 0x1111 is direct: its value (42) lives right after the hash.
        // 0x2222 is a pointer: the 4 bytes after its hash (0x28) are the *address* of
        // the actual data, which happens to be the direct slot's hash position here.
        // Any in-bounds address works for this test, since scan_offsets doesn't care
        // what's stored there, only that it dereferences once.
        let buf = buffer_with_slots(&[(0x1111, 42), (0x2222, 0x28)]);
        let hash_table_end = buf.len();
        let offsets = scan_offsets(
            &buf,
            hash_table_end,
            &[(0x1111, "DIRECT", false), (0x2222, "POINTER", true)],
        )
        .unwrap();
        assert_eq!(buf.read_u32(offsets["DIRECT"]).unwrap(), 42);
        assert_eq!(offsets["POINTER"], 0x28);
    }

    #[test]
    fn resolve_present_hashes_omits_missing_hashes_instead_of_erroring() {
        let buf = buffer_with_slots(&[(0x1111, 42), (0x3333, 30)]);
        let hash_table_end = buf.len();
        let offsets = resolve_present_hashes(&buf, hash_table_end, &[0x1111, 0x2222, 0x3333]).unwrap();
        assert_eq!(offsets.len(), 2);
        assert_eq!(buf.read_u32(offsets[&0x1111]).unwrap(), 42);
        assert_eq!(buf.read_u32(offsets[&0x3333]).unwrap(), 30);
        assert!(!offsets.contains_key(&0x2222));
    }

    #[test]
    fn missing_hash_returns_error() {
        let buf = buffer_with_slots(&[(0x1111, 42)]);
        let hash_table_end = buf.len();
        let result = scan_offsets(&buf, hash_table_end, &[(0x1111, "A", false), (0x9999, "B", false)]);
        assert_eq!(result, Err(SaveError::MissingField("B")));
    }

    #[test]
    fn finds_hash_table_end_at_sentinel() {
        let buf = buffer_with_slots(&[(0x1111, 42), (SENTINEL_HASH, 0)]);
        let end = find_hash_table_end(&buf).unwrap();
        assert_eq!(end, HASH_TABLE_START + 8 + 4);
    }

    #[test]
    fn errors_when_sentinel_not_found() {
        let buf = buffer_with_slots(&[(0x1111, 42)]);
        assert_eq!(find_hash_table_end(&buf), Err(SaveError::UnknownFormat));
    }
}
