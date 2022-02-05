use storage::{BlockIndex, Storage};
use util::{bytes_to_u32, u32_to_bytes, Error};

/// Returns vector of (next_block_index, data_chunk)
fn make_segment_payload_list(
    storage: &Storage,
    data: &[u8],
) -> Result<(Vec<(BlockIndex, Vec<u8>)>, BlockIndex), Error> {
    let block_len = storage.block_len();
    let block_required = data.len() as BlockIndex / block_len;
    let block_indexes = storage.search_block_allocation_indexes(block_required);
    if (block_indexes.len() as BlockIndex) < block_required {
        return Err(Error {
            code: 1,
            message: "Not enough space for log".to_string(),
        });
    }
    let segment_payloads = data
        .chunks(block_len as usize)
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
    return Ok((segment_payloads, first_block_index));
}

/// Add new log to storage with new block index
pub fn create_log(storage: &mut Storage, data: &[u8]) -> Result<BlockIndex, Error> {
    let result = make_segment_payload_list(storage, data);
    if result.is_err() {
        return Err(result.unwrap_err());
    }
    let (payload_list, first_block_index) = result.unwrap();

    for (block_index, segment_payload) in payload_list.iter() {
        let result = storage.write_block(*block_index, segment_payload);
        if result.is_err() {
            return Err(Error {
                code: 2,
                message: "Failed to write log".to_string(),
            });
        }
    }

    return Ok(first_block_index);
}
