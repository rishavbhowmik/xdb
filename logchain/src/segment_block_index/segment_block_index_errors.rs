use util::error::{Error, ErrorType};

pub fn block_index_from_buffer_insufficient_buffer_size(buffer_size: usize) -> Error {
    Error::new(
        ErrorType::Critical,
        "block_index_from_buffer_insufficient_buffer_size",
        Some(format!(
            "Possible logical error or storage corrupt: Insufficient buffer size:\n\t Buffer size: {}",
            buffer_size
        )),
    )
}
