pub struct Error {
    pub code: i32,
    pub message: String,
}
// add fmt::Debug trait to Error struct
use std::fmt;
impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error {{ code: {}, message: {} }}",
            self.code, self.message
        )
    }
}

/// convert 4 bytes unsinged integer little endian bytes array
pub fn u32_to_bytes(n: u32) -> ([u8; 4]) {
    // block_size is in bytes as little endian
    let mut bytes = [0u8; 4];
    bytes[3] = (n >> 24) as u8;
    bytes[2] = (n >> 16) as u8;
    bytes[1] = (n >> 8) as u8;
    bytes[0] = (n >> 0) as u8;
    bytes
}

/// convert little endian bytes array to 4 bytes unsinged integer
pub fn bytes_to_u32(bytes: &[u8]) -> u32 {
    let mut n: u32 = 0;
    n |= (bytes[0] as u32) << 0;
    n |= (bytes[1] as u32) << 8;
    n |= (bytes[2] as u32) << 16;
    n |= (bytes[3] as u32) << 24;
    n
}

pub fn make_chunks(data: &[u8], chunk_len: usize) -> (usize, std::slice::Chunks<u8>) {
    let chunks = data.chunks(chunk_len);
    let blocks_required = data.len() / chunk_len as usize
        + if (data.len() % chunk_len as usize) > 0 {
            1 as usize
        } else {
            0 as usize
        }; // same as chunks.clone().count()
    (blocks_required, chunks)
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u32_to_bytes() {
        assert_eq!(u32_to_bytes(0x12345678), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_bytes_to_u32() {
        assert_eq!(bytes_to_u32(&[0x78, 0x56, 0x34, 0x12]), 0x12345678);
    }

    #[test]
    fn test_u32_to_bytes_and_back() {
        // max u32
        let n = 4294967295;
        let bytes = u32_to_bytes(n);
        let n2 = bytes_to_u32(&bytes);
        assert_eq!(n, n2);
        // min u32
        let n = 0;
        let bytes = u32_to_bytes(n);
        let n2 = bytes_to_u32(&bytes);
        assert_eq!(n, n2);
        // even value
        let n = 2147483648;
        let bytes = u32_to_bytes(n);
        let n2 = bytes_to_u32(&bytes);
        assert_eq!(n, n2);
        // odd value
        let n = 2147483647;
        let bytes = u32_to_bytes(n);
        let n2 = bytes_to_u32(&bytes);
        assert_eq!(n, n2);
    }

    #[test]
    fn test_make_chunks() {
        let data: [u8; 16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let chunk_size = 10;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 2);
        assert_eq!(chunks.clone().count(), 2);
        assert_eq!(chunks.next().unwrap(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(chunks.next().unwrap(), &[11, 12, 13, 14, 15, 16]);
        assert_eq!(chunks.next(), None);

        let data: [u8; 0] = [];
        let chunk_size = 1;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 0);
        assert_eq!(chunks.clone().count(), 0);
        assert_eq!(chunks.next(), None);

        let data: [u8; 1] = [1];
        let chunk_size = 1;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 1);
        assert_eq!(chunks.clone().count(), 1);
        assert_eq!(chunks.next().unwrap(), &[1]);
        assert_eq!(chunks.next(), None);

        let data: [u8; 2] = [1, 2];
        let chunk_size = 1;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 2);
        assert_eq!(chunks.clone().count(), 2);
        assert_eq!(chunks.next().unwrap(), &[1]);
        assert_eq!(chunks.next().unwrap(), &[2]);
        assert_eq!(chunks.next(), None);

        let data: [u8; 3] = [1, 2, 3];
        let chunk_size = 1;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 3);
        assert_eq!(chunks.clone().count(), 3);
        assert_eq!(chunks.next().unwrap(), &[1]);
        assert_eq!(chunks.next().unwrap(), &[2]);
        assert_eq!(chunks.next().unwrap(), &[3]);
        assert_eq!(chunks.next(), None);
    }
}
