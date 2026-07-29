//! Typed lookup over TOTK's full hash dictionary (`zelda-totk.hashes.csv`, vendored from the
//! same MIT-licensed source repo as everything else in this crate). The only thing this module
//! answers is "is this hash's stored value in the hash table a pointer (an address into the blob
//! heap) or a direct scalar value" — needed to safely relocate pointers when
//! `totk::hashtable::insert_and_relocate` grows the file. Field names and the full type
//! vocabulary are parsed but not exposed; nothing else needs them yet.

use std::collections::HashMap;
use std::sync::OnceLock;

const RAW_CSV: &str = include_str!("../../data/totk-hashes.csv");

/// Every TOTK hash type whose stored value is larger than 4 bytes or variable-length, and is
/// therefore pointer-indirected rather than stored directly in the hash table. Matches the
/// source's own `Variable.getVariableSize`, which throws for any type outside this same set
/// (plus the 4-byte direct types `Bool`/`Int`/`UInt`/`Enum`/`Float`, which are deliberately
/// *not* listed here — see `is_pointer`'s doc comment for why unrecognized types must default
/// to "not a pointer", not the reverse).
const POINTER_TYPES: &[&str] = &[
    "IntArray",
    "BoolArray",
    "EnumArray",
    "UIntArray",
    "FloatArray",
    "Vector2",
    "Vector2Array",
    "Vector3",
    "Vector3Array",
    "Vector4",
    "Vector4Array",
    "String",
    "StringArray",
    "String32",
    "String32Array",
    "String64",
    "String64Array",
    "String256",
    "String256Array",
    "UInt64",
    "UInt64Array",
    "WString16",
    "WString16Array",
    "BinaryArray",
];

fn parse_dictionary() -> HashMap<u32, bool> {
    let mut map = HashMap::new();
    for line in RAW_CSV.lines() {
        if line.starts_with('#') || line.starts_with("EnumValues;") || line.trim().is_empty() {
            continue;
        }
        let mut fields = line.splitn(3, ';');
        let (Some(hash_str), Some(type_str)) = (fields.next(), fields.next()) else {
            continue;
        };
        let Ok(hash) = u32::from_str_radix(hash_str, 16) else {
            continue;
        };
        map.insert(hash, POINTER_TYPES.contains(&type_str));
    }
    map
}

fn dictionary() -> &'static HashMap<u32, bool> {
    static DICT: OnceLock<HashMap<u32, bool>> = OnceLock::new();
    DICT.get_or_init(parse_dictionary)
}

/// Whether `hash`'s stored value in the TOTK hash table is a pointer into the blob heap.
/// Unrecognized hashes (not present in the dictionary, or present with an unrecognized type
/// string) default to `false` — deliberately: a scalar `Enum` field's value is itself a full
/// 32-bit hash, so a "default to pointer" policy would risk incrementing real enum state data
/// any time that hash numerically exceeded an insertion point, a much wider blast radius than
/// the narrower failure of a rare unknown field's own pointer going stale.
pub fn is_pointer(hash: u32) -> bool {
    dictionary().get(&hash).copied().unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::totk::murmur3::hash32;

    #[test]
    fn classifies_known_direct_and_pointer_fields_correctly() {
        // Pouch.Weapon.ValidNum is pointer-indirected (IntArray) in the real dictionary,
        // matching this crate's own totk::mod::HASHES table, which marks it is_pointer: true.
        assert!(is_pointer(hash32("Pouch.Weapon.ValidNum")));
        // MaxLife is a direct 4-byte Int, stored inline, never pointer-indirected.
        assert!(!is_pointer(hash32("PlayerStatus.MaxLife")));
    }

    #[test]
    fn unrecognized_hash_defaults_to_not_a_pointer() {
        assert!(!is_pointer(0xdeadbeef));
    }

    #[test]
    fn dictionary_parses_a_realistic_row_count() {
        // Sanity bound, not an exact count (the vendored file may gain/lose a handful of rows
        // upstream) — catches a totally broken parse (e.g. wrong delimiter) immediately.
        assert!(dictionary().len() > 20_000, "expected tens of thousands of parsed hashes, got {}", dictionary().len());
    }
}
