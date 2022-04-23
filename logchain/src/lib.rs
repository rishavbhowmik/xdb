use storage::{BlockIndex, Storage};
use util::error::Error;
use util::make_chunks;

mod logchain_errors;

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
    if blocks_required == 0 {
        let block_indexes = storage.search_block_allocation_indexes(1);
        if block_indexes.is_empty() {
            return Err(
                logchain_errors::make_segment_payload_list_insufficient_blocks(blocks_required),
            );
        }
        let block_index = block_indexes[0];
        return Ok((
            vec![(
                block_indexes[0],
                [data, &block_index_to_buffer(LAST_NEXT_BLOCK_INDEX)].concat(),
            )],
            block_index,
            block_index,
        ));
    }
    let block_indexes = storage.search_block_allocation_indexes(blocks_required as BlockIndex);
    if block_indexes.len() < blocks_required {
        return Err(
            logchain_errors::make_segment_payload_list_insufficient_blocks(blocks_required),
        );
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
    Ok((segment_payloads, first_block_index, last_block_index))
}

/// Add new log to storage with new block index
/// - Returns (first_block_index, last_block_index)
pub fn create_log(storage: &mut Storage, data: &[u8]) -> Result<(BlockIndex, BlockIndex), Error> {
    let (payload_list, first_block_index, last_block_index) =
        make_segment_payload_list(storage, data)?;
    for (block_index, segment_payload) in payload_list.iter() {
        storage.write_block(*block_index, segment_payload)?;
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
        let (_, segment_payload) = storage.read_block(last_block_index)?;

        // parse next block index
        let next_block_index = block_index_from_buffer(&segment_payload)?;

        // check if last block
        if next_block_index != LAST_NEXT_BLOCK_INDEX {
            last_block_index = next_block_index;
        } else {
            // reached last block
            let existing_last_block_void_size =
                storage.block_len() as usize - segment_payload.len();

            // - segment payload list with remaining data if any
            let (payload_list, new_next_block_index, new_last_block_index) =
                if (data.len() as isize - existing_last_block_void_size as isize) > 0 {
                    make_segment_payload_list(storage, &data[existing_last_block_void_size..])?
                } else {
                    (vec![], LAST_NEXT_BLOCK_INDEX, last_block_index)
                };

            // - update next_block_index of existing last segment
            let existing_last_segment_new_block_data = [
                &block_index_to_buffer(new_next_block_index),
                &segment_payload[4..],
                &data[0..(if existing_last_block_void_size == 0 {
                    0
                } else if existing_last_block_void_size > data.len() {
                    data.len()
                } else {
                    existing_last_block_void_size
                })],
            ]
            .concat();

            // - write updated last block
            storage.write_block(last_block_index, &existing_last_segment_new_block_data)?;

            // - write new blocks
            for (block_index, segment_payload) in payload_list.iter() {
                storage.write_block(*block_index, segment_payload)?;
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
        // read block
        let (_, segment_payload) = storage.read_block(block_index_cache)?;

        // parse next block index
        let next_block_index = block_index_from_buffer(&segment_payload)?;

        // delete block
        storage.delete_block(block_index_cache, hard_delete)?;

        // check if reached last block
        if next_block_index != LAST_NEXT_BLOCK_INDEX {
            block_index_cache = next_block_index;
        } else {
            storage.delete_block(block_index_cache, hard_delete)?;
            return Ok((start_segment_block_index, block_index_cache));
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
        // read block
        let (_, segment_payload) = storage.read_block(block_index_cache)?;

        // parse next block index
        let next_block_index = block_index_from_buffer(&segment_payload)?;

        // append segment payload to log data
        log_data.extend_from_slice(&segment_payload[4..]);
        if next_block_index != LAST_NEXT_BLOCK_INDEX {
            block_index_cache = next_block_index;
        } else {
            return Ok((start_segment_block_index, block_index_cache, log_data));
        }
    }
}
