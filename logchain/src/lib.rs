use storage::{BlockIndex, Storage, BLOCK_HEADER_SIZE};
use util::{bytes_to_u32, u32_to_bytes, Error};

const BLOCK_INDEX_SIZE: usize = std::mem::size_of::<BlockIndex>();
/// Returns (Vector(next_block_index, data_chunk), first_block_index, last_block_index)
pub fn make_segment_payload_list(
    storage: &Storage,
    data: &[u8],
) -> Result<(Vec<(BlockIndex, Vec<u8>)>, BlockIndex, BlockIndex), Error> {
    let block_len = storage.block_len() as usize;
    let chunk_len = block_len - BLOCK_INDEX_SIZE;
    let chunks = data.chunks(chunk_len);
    let block_required = data.len() / chunk_len as usize
        + if (data.len() % chunk_len as usize) > 0 {
            1 as usize
        } else {
            0 as usize
        }; // same as chunks.len()
    let block_indexes = storage.search_block_allocation_indexes(block_required as BlockIndex);
    if block_indexes.len() < block_required {
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
                storage.block_len() as usize - BLOCK_HEADER_SIZE - segment_payload.len();
            // - segment payload list with remaining data
            let payload_list_result =
                make_segment_payload_list(storage, &data[existing_last_block_void_size..]);
            if payload_list_result.is_err() {
                return Err(payload_list_result.unwrap_err());
            }
            let (payload_list, new_next_block_index, new_last_block_index) =
                payload_list_result.unwrap();
            // - update next_block_index of existing last segment
            let existing_last_segment_new_block_data = [
                &u32_to_bytes(new_next_block_index),
                &segment_payload[4..],
                &data[0..(existing_last_block_void_size)],
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
