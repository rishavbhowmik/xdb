#[derive(Clone, Copy)]
pub enum PageUsizeType {
    U8,
    U16,
    U32,
    U64,
}

impl PageUsizeType {
    /// Returns PageUsizeType with enough bytes to store the given usize
    pub fn for_max_usize(max: usize) -> Self {
        if max <= u8::MAX as usize {
            PageUsizeType::U8
        } else if max <= u16::MAX as usize {
            PageUsizeType::U16
        } else if max <= u32::MAX as usize {
            PageUsizeType::U32
        } else {
            PageUsizeType::U64
        }
    }
    /// Size of the PageUsizeType
    pub fn size_of(page_size_type: PageUsizeType) -> usize {
        match page_size_type {
            PageUsizeType::U8 => 1,
            PageUsizeType::U16 => 2,
            PageUsizeType::U32 => 4,
            PageUsizeType::U64 => 8,
        }
    }
}

/// Desearialize page usize from little endian bytes
pub fn page_usize_from_le_bytes(bytes: &[u8], page_size_type: PageUsizeType) -> usize {
    match page_size_type {
        PageUsizeType::U8 => u8::from_le_bytes([bytes[0]]) as usize,
        PageUsizeType::U16 => u16::from_le_bytes([bytes[0], bytes[1]]) as usize,
        PageUsizeType::U32 => u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize,
        PageUsizeType::U64 => u64::from_le_bytes([
            bytes[0], bytes[1], bytes[2], bytes[3], bytes[4], bytes[5], bytes[6], bytes[7],
        ]) as usize,
    }
}

/// Serialize page usize to little endian bytes
pub fn page_usize_to_le_bytes(value: usize, page_size_type: PageUsizeType) -> Vec<u8> {
    match page_size_type {
        PageUsizeType::U8 => u8::to_le_bytes(value as u8).to_vec(),
        PageUsizeType::U16 => u16::to_le_bytes(value as u16).to_vec(),
        PageUsizeType::U32 => u32::to_le_bytes(value as u32).to_vec(),
        PageUsizeType::U64 => u64::to_le_bytes(value as u64).to_vec(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test PageUsizeType::for_max_usize

    #[test]
    fn test_page_usize_type_for_max_usize() {
        assert!(matches!(PageUsizeType::for_max_usize(0), PageUsizeType::U8));
        assert!(matches!(
            PageUsizeType::for_max_usize(u8::MAX as usize),
            PageUsizeType::U8
        ));
        assert!(matches!(
            PageUsizeType::for_max_usize(u8::MAX as usize + 1),
            PageUsizeType::U16
        ));
        assert!(matches!(
            PageUsizeType::for_max_usize(u16::MAX as usize),
            PageUsizeType::U16
        ));
        assert!(matches!(
            PageUsizeType::for_max_usize(u16::MAX as usize + 1),
            PageUsizeType::U32
        ));
        assert!(matches!(
            PageUsizeType::for_max_usize(u32::MAX as usize),
            PageUsizeType::U32
        ));
        assert!(matches!(
            PageUsizeType::for_max_usize(u32::MAX as usize + 1),
            PageUsizeType::U64
        ));
        assert!(matches!(
            PageUsizeType::for_max_usize(u64::MAX as usize),
            PageUsizeType::U64
        ));
    }

    // Test PageUsizeType::size_of

    #[test]
    fn test_page_usize_type_size_of() {
        assert_eq!(PageUsizeType::size_of(PageUsizeType::U8), 1);
        assert_eq!(PageUsizeType::size_of(PageUsizeType::U16), 2);
        assert_eq!(PageUsizeType::size_of(PageUsizeType::U32), 4);
        assert_eq!(PageUsizeType::size_of(PageUsizeType::U64), 8);
    }

    // Test page_usize_from_le_bytes()

    #[test]
    fn test_page_usize_from_le_bytes_u8() {
        // Test min
        let value = u8::MIN as usize;
        let bytes = u8::to_le_bytes(value as u8); // [0]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U8);
        assert_eq!(result, value);
        // Test max
        let value = u8::MAX as usize;
        let bytes = u8::to_le_bytes(value as u8); // [255]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U8);
        assert_eq!(result, value);
        // Test median
        let value = u8::MAX as usize / 2;
        let bytes = u8::to_le_bytes(value as u8); // [127]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U8);
        assert_eq!(result, value);
    }

    #[test]
    fn test_page_usize_from_le_bytes_u16() {
        // Test min
        let value = u16::MIN as usize;
        let bytes = u16::to_le_bytes(value as u16); // [0, 0]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U16);
        assert_eq!(result, value);
        // Test max
        let value = u16::MAX as usize;
        let bytes = u16::to_le_bytes(value as u16); // [255, 255]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U16);
        assert_eq!(result, value);
        // Test median
        let value = u16::MAX as usize / 2;
        let bytes = u16::to_le_bytes(value as u16); // [127, 127]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U16);
        assert_eq!(result, value);
    }

    #[test]
    fn test_page_usize_from_le_bytes_u32() {
        // Test min
        let value = u32::MIN as usize;
        let bytes = u32::to_le_bytes(value as u32); // [0, 0, 0, 0]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U32);
        assert_eq!(result, value);
        // Test max
        let value = u32::MAX as usize;
        let bytes = u32::to_le_bytes(value as u32); // [255, 255, 255, 255]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U32);
        assert_eq!(result, value);
        // Test median
        let value = u32::MAX as usize / 2;
        let bytes = u32::to_le_bytes(value as u32); // [127, 127, 127, 127]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U32);
        assert_eq!(result, value);
    }

    #[test]
    fn test_page_usize_from_le_bytes_u64() {
        // Test min
        let value = u64::MIN as usize;
        let bytes = u64::to_le_bytes(value as u64); // [0, 0, 0, 0, 0, 0, 0, 0]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U64);
        assert_eq!(result, value);
        // Test max
        let value = u64::MAX as usize;
        let bytes = u64::to_le_bytes(value as u64); // [255, 255, 255, 255, 255, 255, 255, 255]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U64);
        assert_eq!(result, value);
        // Test median
        let value = u64::MAX as usize / 2;
        let bytes = u64::to_le_bytes(value as u64); // [127, 127, 127, 127, 127, 127, 127, 127]
        let result = page_usize_from_le_bytes(&bytes, PageUsizeType::U64);
        assert_eq!(result, value);
    }

    // Test page_usize_to_le_bytes()

    #[test]
    fn test_page_usize_to_le_bytes_u8() {
        // Test min
        let value = u8::MIN as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U8);
        assert_eq!(result, u8::to_le_bytes(value as u8)); // [0]

        // Test max
        let value = u8::MAX as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U8);
        assert_eq!(result, u8::to_le_bytes(value as u8)); // [255]

        // Test median
        let value = u8::MAX as usize / 2;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U8);
        assert_eq!(result, u8::to_le_bytes(value as u8)); // [127]
    }

    #[test]
    fn test_page_usize_to_le_bytes_u16() {
        // Test min
        let value = u16::MIN as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U16);
        assert_eq!(result, u16::to_le_bytes(value as u16)); // [0, 0]

        // Test max
        let value = u16::MAX as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U16);
        assert_eq!(result, u16::to_le_bytes(value as u16)); // [255, 255]

        // Test median
        let value = u16::MAX as usize / 2;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U16);
        assert_eq!(result, u16::to_le_bytes(value as u16)); // [127, 127]
    }

    #[test]
    fn test_page_usize_to_le_bytes_u32() {
        // Test min
        let value = u32::MIN as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U32);
        assert_eq!(result, u32::to_le_bytes(value as u32)); // [0, 0, 0, 0]

        // Test max
        let value = u32::MAX as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U32);
        assert_eq!(result, u32::to_le_bytes(value as u32)); // [255, 255, 255, 255]

        // Test median
        let value = u32::MAX as usize / 2;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U32);
        assert_eq!(result, u32::to_le_bytes(value as u32)); // [127, 127, 127, 127]
    }

    #[test]
    fn test_page_usize_to_le_bytes_u64() {
        // Test min
        let value = u64::MIN as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U64);
        assert_eq!(result, u64::to_le_bytes(value as u64)); // [0, 0, 0, 0, 0, 0, 0, 0]

        // Test max
        let value = u64::MAX as usize;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U64);
        assert_eq!(result, u64::to_le_bytes(value as u64)); // [255, 255, 255, 255, 255, 255, 255, 255]

        // Test median
        let value = u64::MAX as usize / 2;
        let result = page_usize_to_le_bytes(value, PageUsizeType::U64);
        assert_eq!(result, u64::to_le_bytes(value as u64)); // [127, 127, 127, 127, 127, 127, 127, 127]
    }
}
