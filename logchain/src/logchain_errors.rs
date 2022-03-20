use util::error::{Error, ErrorType};

// ... ... make_segment_payload_list ... ...
pub fn make_segment_payload_list_insufficient_blocks(required_blocks: usize) -> Error {
    Error::new(
        ErrorType::Happens,
        "make_segment_payload_list_insufficient_blocks",
        Some(format!(
            "Insufficient block allocation:\n\t Required blocks: {}",
            required_blocks
        )),
    )
}
