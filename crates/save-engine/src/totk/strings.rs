use crate::binary::SaveBuffer;
use crate::error::SaveError;

pub const STRING64_SIZE: usize = 0x40;

/// Reads a null-terminated ASCII string from a single fixed 0x40-byte block — TOTK's
/// `String64` type. Simpler than BOTW's 8-byte-strided padded strings: no interleaved
/// padding here, just one contiguous block.
pub fn read_string64(buf: &SaveBuffer, offset: usize) -> String {
    buf.read_string(offset, STRING64_SIZE)
}

/// Writes `value`'s bytes into a 0x40-byte block starting at `offset`, zero-padding any
/// remaining bytes. Mirrors `tempFile.writeString(offset, value, 0x40)`.
///
/// Assumes ASCII strings, true for TOTK's checkpoint/location identifiers and the same
/// assumption BOTW's padded-string helpers make.
pub fn write_string64(buf: &mut SaveBuffer, offset: usize, value: &str) -> Result<(), SaveError> {
    let bytes = value.as_bytes();
    for i in 0..STRING64_SIZE {
        buf.write_u8(offset + i, bytes.get(i).copied().unwrap_or(0))?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_a_short_string() {
        let mut buf = SaveBuffer::new(vec![0xffu8; STRING64_SIZE]);
        write_string64(&mut buf, 0, "MainField").unwrap();
        assert_eq!(read_string64(&buf, 0), "MainField");
    }

    #[test]
    fn write_pads_remaining_bytes_with_zero() {
        let mut buf = SaveBuffer::new(vec![0xffu8; STRING64_SIZE]);
        write_string64(&mut buf, 0, "Hi").unwrap();
        assert_eq!(buf.read_u8(2).unwrap(), 0);
        assert_eq!(buf.read_u8(STRING64_SIZE - 1).unwrap(), 0);
    }

    #[test]
    fn write_out_of_bounds_returns_err_instead_of_panicking() {
        let mut buf = SaveBuffer::new(vec![0u8; 4]);
        assert!(write_string64(&mut buf, 0, "too long for this four-byte buffer").is_err());
    }
}
