use storage::{BlockIndex, Storage, BLOCK_HEADER_SIZE};
use util::{bytes_to_u32, u32_to_bytes, Error};

const BLOCK_INDEX_SIZE: usize = std::mem::size_of::<BlockIndex>();

/// Returns (blocks_required, chunks)
fn make_chunks(data: &[u8], chunk_len: usize) -> (usize, std::slice::Chunks<u8>) {
    let chunks = data.chunks(chunk_len);
    let blocks_required = data.len() / chunk_len as usize
        + if (data.len() % chunk_len as usize) > 0 {
            1 as usize
        } else {
            0 as usize
        }; // same as chunks.clone().count()
    (blocks_required, chunks)
}

/// Returns (Vector(next_block_index, data_chunk), first_block_index, last_block_index)
pub fn make_segment_payload_list(
    storage: &Storage,
    data: &[u8],
) -> Result<(Vec<(BlockIndex, Vec<u8>)>, BlockIndex, BlockIndex), Error> {
    let block_len = storage.block_len() as usize;
    let chunk_len = block_len - BLOCK_INDEX_SIZE;
    let (blocks_required, chunks) = make_chunks(data, chunk_len);
    let block_indexes = storage.search_block_allocation_indexes(blocks_required as BlockIndex);
    if block_indexes.len() < blocks_required {
        return Err(Error {
            code: 1,
            message: "Not enough space for log".to_string(),
        });
    }
    let segment_payloads = chunks
        .enumerate()
        .map(|(block_index, data_chunk)| {
            (
                block_indexes[block_index],
                [
                    &u32_to_bytes(if block_index < block_indexes.len() - 1 {
                        block_indexes[block_index + 1]
                    } else {
                        BlockIndex::MAX // for last block
                    }),
                    data_chunk,
                ]
                .concat(),
            )
        })
        .collect::<Vec<_>>();

    let first_block_index = block_indexes[0];
    let last_block_index = block_indexes[block_indexes.len() - 1];
    return Ok((segment_payloads, first_block_index, last_block_index));
}

/// Add new log to storage with new block index
/// - Returns (first_block_index, last_block_index)
pub fn create_log(storage: &mut Storage, data: &[u8]) -> Result<(BlockIndex, BlockIndex), Error> {
    let result = make_segment_payload_list(storage, data);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let (payload_list, first_block_index, last_block_index) = result.unwrap();
    for (block_index, segment_payload) in payload_list.iter() {
        let result = storage.write_block(*block_index, segment_payload);
        if result.is_err() {
            return Err(Error {
                code: 2,
                message: "Failed to write log".to_string(),
            });
        }
    }
    Ok((first_block_index, last_block_index))
}

/// Append existing log to storage with new block index
/// - appends last block with 1st chunk of data
/// - store remaining chunks of data in new blocks
/// - Returns last_block_index
pub fn append_log(
    storage: &mut Storage,
    block_index: BlockIndex,
    data: &[u8],
) -> Result<BlockIndex, Error> {
    // traverse to last block of log
    let mut last_block_index = block_index; // no necessarily 1st or last block of log, prefer last block to elemenate search time
    loop {
        let result = storage.read_block(last_block_index);
        if result.is_err() {
            return Err(Error {
                code: 3,
                message: "Failed to read log".to_string(),
            });
        }
        let (_, segment_payload) = result.unwrap();
        let next_block_index = bytes_to_u32(&segment_payload[0..4]);
        if next_block_index == BlockIndex::MAX {
            // reached last block
            let existing_last_block_void_size =
                storage.block_len() as usize - segment_payload.len();
            // - segment payload list with remaining data if any
            let payload_list_result =
                if (data.len() as isize - existing_last_block_void_size as isize) > 0 {
                    make_segment_payload_list(storage, &data[existing_last_block_void_size..])
                } else {
                    Ok((vec![], BlockIndex::MAX, last_block_index))
                };
            if payload_list_result.is_err() {
                return Err(payload_list_result.unwrap_err());
            }
            let (payload_list, new_next_block_index, new_last_block_index) =
                payload_list_result.unwrap();
            // - update next_block_index of existing last segment
            let existing_last_segment_new_block_data = [
                &u32_to_bytes(new_next_block_index),
                &segment_payload[4..],
                &data[0..(if existing_last_block_void_size == 0 {
                    0 as usize
                } else if existing_last_block_void_size > data.len() {
                    data.len()
                } else {
                    existing_last_block_void_size as usize
                })],
            ]
            .concat();
            // - write updated last block
            let write_result =
                storage.write_block(last_block_index, &existing_last_segment_new_block_data);
            if write_result.is_err() {
                return Err(Error {
                    code: 4,
                    message: "Failed to write log".to_string(),
                });
            }
            // - write new blocks
            for (block_index, segment_payload) in payload_list.iter() {
                let write_result = storage.write_block(*block_index, segment_payload);
                if write_result.is_err() {
                    return Err(Error {
                        code: 5,
                        message: "Failed to write log".to_string(),
                    });
                }
            }
            return Ok(new_last_block_index);
        } else {
            last_block_index = next_block_index;
        }
    }
}

/// Delete log from storage
/// - Returns (first_block_index, last_block_index)
pub fn delete_log(
    storage: &mut Storage,
    start_segment_block_index: BlockIndex,
    hard_delete: bool,
) -> Result<(BlockIndex, BlockIndex), Error> {
    let mut block_index_cache = start_segment_block_index;
    loop {
        let result = storage.read_block(block_index_cache);
        if result.is_err() {
            return Err(Error {
                code: 6,
                message: "Failed to read log".to_string(),
            });
        }
        let (_, segment_payload) = result.unwrap();
        let next_block_index = bytes_to_u32(&segment_payload[0..4]);
        if next_block_index == BlockIndex::MAX {
            // reached last block
            let delete_result = storage.delete_block(next_block_index, hard_delete);
            if delete_result.is_err() {
                return Err(Error {
                    code: 7,
                    message: "Failed to delete log".to_string(),
                });
            } else {
                return Ok((start_segment_block_index, block_index_cache));
            }
        } else {
            let delete_result = storage.delete_block(next_block_index, hard_delete);
            if delete_result.is_err() {
                return Err(Error {
                    code: 8,
                    message: "Failed to delete log".to_string(),
                });
            }
            block_index_cache = next_block_index;
        }
    }
}

// unit tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_chunks() {
        let data: [u8;16] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
        let chunk_size = 10;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 2);
        assert_eq!(chunks.clone().count(), 2);
        assert_eq!(chunks.next().unwrap(), &[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        assert_eq!(chunks.next().unwrap(), &[11, 12, 13, 14, 15, 16]);
        assert_eq!(chunks.next(), None);

        let data: [u8;0] = [];
        let chunk_size = 1;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 0);
        assert_eq!(chunks.clone().count(), 0);
        assert_eq!(chunks.next(), None);

        let data: [u8;1] = [1];
        let chunk_size = 1;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 1);
        assert_eq!(chunks.clone().count(), 1);
        assert_eq!(chunks.next().unwrap(), &[1]);
        assert_eq!(chunks.next(), None);

        let data: [u8;2] = [1, 2];
        let chunk_size = 1;
        let (blocks_required, mut chunks) = make_chunks(&data, chunk_size);
        assert_eq!(blocks_required, 2);
        assert_eq!(chunks.clone().count(), 2);
        assert_eq!(chunks.next().unwrap(), &[1]);
        assert_eq!(chunks.next().unwrap(), &[2]);
        assert_eq!(chunks.next(), None);

        let data: [u8;3] = [1, 2, 3];
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