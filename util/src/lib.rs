pub enum Env {
    Dev,
    Test,
    Prod,
}

/// Enviornment to compile for
pub const ENV: Env = Env::Dev;

pub mod byte_cursor;
pub mod error;
pub mod test_util;

pub fn make_chunks(data: &[u8], chunk_len: usize) -> (usize, std::slice::Chunks<u8>) {
    let chunks = data.chunks(chunk_len);
    let blocks_required = data.len() / chunk_len as usize
        + if (data.len() % chunk_len as usize) > 0 {
            1
        } else {
            0
        }; // same as chunks.clone().count()
    (blocks_required, chunks)
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

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
