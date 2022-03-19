use storage::{BlockIndex, Storage};
use util::{make_chunks, Error};

mod segment_block_index;
use segment_block_index::{
    block_index_from_buffer, block_index_to_buffer, BLOCK_INDEX_SIZE, LAST_NEXT_BLOCK_INDEX,
};

/// Returns (Vector<(next_block_index, data_chunk)>, first_block_index, last_block_index)
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
                block_indexes[block_index], // block_index to store segment
                [
                    &block_index_to_buffer(if block_index < block_indexes.len() - 1 {
                        block_indexes[block_index + 1]
                    } else {
                        LAST_NEXT_BLOCK_INDEX
                    }), // next block index
                    data_chunk, // segment payload
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
        // parse next block index
        let next_block_index_result = block_index_from_buffer(&segment_payload);
        if next_block_index_result.is_err() {
            return Err(next_block_index_result.unwrap_err());
        }
        let next_block_index = next_block_index_result.unwrap();
        // check if last block
        if next_block_index != LAST_NEXT_BLOCK_INDEX {
            last_block_index = next_block_index;
        } else {
            // reached last block
            let existing_last_block_void_size =
                storage.block_len() as usize - segment_payload.len();
            // - segment payload list with remaining data if any
            let payload_list_result =
                if (data.len() as isize - existing_last_block_void_size as isize) > 0 {
                    make_segment_payload_list(storage, &data[existing_last_block_void_size..])
                } else {
                    Ok((vec![], LAST_NEXT_BLOCK_INDEX, last_block_index))
                };
            if payload_list_result.is_err() {
                return Err(payload_list_result.unwrap_err());
            }
            let (payload_list, new_next_block_index, new_last_block_index) =
                payload_list_result.unwrap();
            // - update next_block_index of existing last segment
            let existing_last_segment_new_block_data = [
                &block_index_to_buffer(new_next_block_index),
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
        let next_block_index_result = block_index_from_buffer(&segment_payload);
        if next_block_index_result.is_err() {
            return Err(next_block_index_result.unwrap_err());
        }
        let next_block_index = next_block_index_result.unwrap();
        // delete block
        let delete_result = storage.delete_block(block_index_cache, hard_delete);
        if delete_result.is_err() {
            return Err(Error {
                code: 7,
                message: "Failed to delete log".to_string(),
            });
        }
        // check if reached last block
        if next_block_index != LAST_NEXT_BLOCK_INDEX {
            block_index_cache = next_block_index;
        } else {
            let delete_result = storage.delete_block(block_index_cache, hard_delete);
            if delete_result.is_err() {
                return Err(Error {
                    code: 8,
                    message: "Failed to delete log".to_string(),
                });
            } else {
                return Ok((start_segment_block_index, block_index_cache));
            }
        }
    }
}

/// Read log from storage
/// - Returns (first_block_index, last_block_index, log_data)
/// - log_data is concatenation of all segments
pub fn read_log(
    storage: &mut Storage,
    start_segment_block_index: BlockIndex,
) -> Result<(BlockIndex, BlockIndex, Vec<u8>), Error> {
    let mut block_index_cache = start_segment_block_index;
    let mut log_data = vec![];
    loop {
        let result = storage.read_block(block_index_cache);
        if result.is_err() {
            return Err(Error {
                code: 9,
                message: "Failed to read log".to_string(),
            });
        }
        let (_, segment_payload) = result.unwrap();
        let next_block_index_result = block_index_from_buffer(&segment_payload);
        if next_block_index_result.is_err() {
            return Err(next_block_index_result.unwrap_err());
        }
        let next_block_index = next_block_index_result.unwrap();
        log_data.extend_from_slice(&segment_payload[4..]);
        if next_block_index != LAST_NEXT_BLOCK_INDEX {
            block_index_cache = next_block_index;
        } else {
            return Ok((start_segment_block_index, block_index_cache, log_data));
        }
    }
}
