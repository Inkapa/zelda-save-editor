use std::collections::HashMap;

use crate::binary::SaveBuffer;

/// Scans `buf` once for each `(hash, name)` pair in `hashes`, in order, and records the
/// byte offset immediately after each match (where that property's value lives).
/// `hashes` must be sorted in ascending hash order matching the file's own layout —
/// the search cursor only ever moves forward.
pub fn scan_offsets(buf: &SaveBuffer, hashes: &[(u32, &'static str)]) -> HashMap<&'static str, usize> {
    let mut offsets = HashMap::new();
    let mut start = 0x0c;
    for &(hash, name) in hashes {
        let mut j = start;
        while j + 8 <= buf.len() {
            if buf.read_u32(j) == hash {
                offsets.insert(name, j + 4);
                start = j + 8;
                break;
            }
            j += 8;
        }
    }
    offsets
}

#[cfg(test)]
mod tests {
    use super::*;

    fn buffer_with_slots(slots: &[(u32, u32)]) -> SaveBuffer {
        let mut data = vec![0u8; 0x0c];
        for (hash, value) in slots {
            data.extend_from_slice(&hash.to_be_bytes());
            data.extend_from_slice(&value.to_be_bytes());
        }
        SaveBuffer::new(data)
    }

    #[test]
    fn finds_offsets_for_known_hashes_in_order() {
        let buf = buffer_with_slots(&[(0x1111, 10), (0x2222, 20), (0x3333, 30)]);
        let offsets = scan_offsets(&buf, &[(0x1111, "A"), (0x2222, "B"), (0x3333, "C")]);
        assert_eq!(offsets.len(), 3);
        assert_eq!(buf.read_u32(offsets["A"]), 10);
        assert_eq!(buf.read_u32(offsets["B"]), 20);
        assert_eq!(buf.read_u32(offsets["C"]), 30);
    }

    #[test]
    fn missing_hash_is_absent_but_does_not_block_later_ones() {
        let buf = buffer_with_slots(&[(0x1111, 10), (0x3333, 30)]);
        let offsets = scan_offsets(&buf, &[(0x1111, "A"), (0x2222, "B"), (0x3333, "C")]);
        assert_eq!(offsets.len(), 2);
        assert!(!offsets.contains_key("B"));
        assert_eq!(buf.read_u32(offsets["C"]), 30);
    }
}
