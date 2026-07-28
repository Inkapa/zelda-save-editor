pub struct SaveBuffer {
    data: Vec<u8>,
    pub little_endian: bool,
}

impl SaveBuffer {
    pub fn new(data: Vec<u8>) -> Self {
        SaveBuffer {
            data,
            little_endian: false,
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn into_bytes(self) -> Vec<u8> {
        self.data
    }

    pub fn read_u8(&self, offset: usize) -> u8 {
        self.data[offset]
    }

    pub fn write_u8(&mut self, offset: usize, val: u8) {
        self.data[offset] = val;
    }

    pub fn read_u32(&self, offset: usize) -> u32 {
        let b = [
            self.data[offset],
            self.data[offset + 1],
            self.data[offset + 2],
            self.data[offset + 3],
        ];
        if self.little_endian {
            u32::from_le_bytes(b)
        } else {
            u32::from_be_bytes(b)
        }
    }

    pub fn write_u32(&mut self, offset: usize, val: u32) {
        let bytes = if self.little_endian {
            val.to_le_bytes()
        } else {
            val.to_be_bytes()
        };
        self.data[offset..offset + 4].copy_from_slice(&bytes);
    }

    pub fn read_f32(&self, offset: usize) -> f32 {
        f32::from_bits(self.read_u32(offset))
    }

    pub fn write_f32(&mut self, offset: usize, val: f32) {
        self.write_u32(offset, val.to_bits());
    }

    /// Mirrors the source project's `MarcFile.readString`: consecutive non-null bytes
    /// starting at `offset`, stopping at the first null byte, `max_len`, or the end of
    /// the buffer.
    pub fn read_string(&self, offset: usize, max_len: usize) -> String {
        let mut out = String::new();
        for i in 0..max_len {
            let pos = offset + i;
            if pos >= self.data.len() {
                break;
            }
            let byte = self.data[pos];
            if byte == 0 {
                break;
            }
            out.push(byte as char);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_and_writes_u32_big_endian() {
        let mut buf = SaveBuffer::new(vec![0u8; 8]);
        buf.write_u32(0, 0x11223344);
        assert_eq!(buf.data, vec![0x11, 0x22, 0x33, 0x44, 0, 0, 0, 0]);
        assert_eq!(buf.read_u32(0), 0x11223344);
    }

    #[test]
    fn reads_and_writes_u32_little_endian() {
        let mut buf = SaveBuffer::new(vec![0u8; 8]);
        buf.little_endian = true;
        buf.write_u32(0, 0x11223344);
        assert_eq!(buf.data, vec![0x44, 0x33, 0x22, 0x11, 0, 0, 0, 0]);
        assert_eq!(buf.read_u32(0), 0x11223344);
    }

    #[test]
    fn read_string_stops_at_null_byte() {
        let buf = SaveBuffer::new(vec![b'H', b'i', 0, b'X']);
        assert_eq!(buf.read_string(0, 4), "Hi");
    }

    #[test]
    fn read_string_stops_at_max_len() {
        let buf = SaveBuffer::new(vec![b'a', b'b', b'c', b'd']);
        assert_eq!(buf.read_string(0, 2), "ab");
    }

    #[test]
    fn f32_round_trips_through_bits() {
        let mut buf = SaveBuffer::new(vec![0u8; 4]);
        buf.write_f32(0, 3.5);
        assert_eq!(buf.read_f32(0), 3.5);
    }
}
