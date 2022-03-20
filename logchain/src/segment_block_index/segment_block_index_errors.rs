use util::error::{Error, ErrorType};

pub fn block_index_from_buffer_insufficient_buffer_size(buffer_size: usize) -> Error {
    Error::new(
        ErrorType::Happens,
        "block_index_from_buffer_insufficient_buffer_size",
        Some(format!(
            "Insufficient buffer size:\n\t Buffer size: {}",
            buffer_size
        )),
    )
}
