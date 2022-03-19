use storage::BlockIndex;
use util::Error;

pub const BLOCK_INDEX_SIZE: usize = std::mem::size_of::<BlockIndex>();

/// Value of next block index in last block of the chain.
pub const LAST_NEXT_BLOCK_INDEX: BlockIndex = BlockIndex::MAX;

pub fn block_index_from_buffer(buffer: &[u8]) -> Result<BlockIndex, Error> {
    if buffer.len() < BLOCK_INDEX_SIZE {
        return Err(Error {
            code: 2,
            message: "Insufficient buffer size".to_string(),
        });
    }
    Ok(BlockIndex::from_le_bytes([
        buffer[0], buffer[1], buffer[2], buffer[3],
    ]))
}

pub fn block_index_to_buffer(next_block_index: BlockIndex) -> [u8; BLOCK_INDEX_SIZE] {
    BlockIndex::to_le_bytes(next_block_index)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_index_to_buffer() {
        assert_eq!(block_index_to_buffer(0x12345678), [0x78, 0x56, 0x34, 0x12]);
    }

    #[test]
    fn test_block_index_from_buffer() {
        // Normal case
        let result = block_index_from_buffer(&[0x78, 0x56, 0x34, 0x12]);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), 0x12345678);
        // Insufficient buffer size case
        let result = block_index_from_buffer(&[0x78, 0x56, 0x34]);
        assert_eq!(result.is_err(), true);
        assert_eq!(result.unwrap_err().code, 2);
    }

    #[test]
    fn test_block_index_to_buffer_and_back() {
        // max u32
        let n = BlockIndex::MAX;
        let bytes = block_index_to_buffer(n);
        let result = block_index_from_buffer(&bytes);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), n);
        // min u32
        let n = BlockIndex::MIN;
        let bytes = block_index_to_buffer(n);
        let result = block_index_from_buffer(&bytes);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), n);
        // even value
        let n = (BlockIndex::MAX / 4) * 2;
        let bytes = block_index_to_buffer(n);
        let result = block_index_from_buffer(&bytes);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), n);
        // odd value
        let n = (BlockIndex::MAX / 4) * 2 + 1;
        let bytes = block_index_to_buffer(n);
        let result = block_index_from_buffer(&bytes);
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap(), n);
    }
}
