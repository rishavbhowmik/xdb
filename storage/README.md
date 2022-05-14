# Storage Engine

## Idea

### Storage

Maintain a storage file, segmented in blocks of fixed length.

> Maximum size of data_payload a block can store is `BLOCK_LEN`. which is fixed for all blocks in the storage.

Each block has a serial index, starting from 0.

Each block stores data_payload_size as 4 bytes unsigned int and data_payload.

> Each block can store a data payload of size less or equal to `BLOCK_LEN`. The size of the payload is `block_data_size`.

Blocks with `block_data_size` 0 are considered as **free blocks** and can be used to write new data.

If all blocks in storage are not free, the storage file is extended with new blocks.

## Design

### File structure

```
|----------------------------|
| BLOCK_LEN        <4 Bytes> | <- Storage header
|----------------------------|
| Block 1 dataSize <4 Bytes> | <- Block header
|----------------------------|
| Block 1 Data    <BLOCK_LEN>| <- Block data
|----------------------------|
| Block 2 dataSize <4 Bytes> | <- Block header
|----------------------------|
| Block 2 Data    <BLOCK_LEN>| <- Block data
|----------------------------|
| so on...                   |
```

### Free blocks

Blocks with data_length 0, which can be reused to store new data.

#### Free blocks array in memory

- Initialize free blocks with all blocks in the file with data_length 0.
- When a block is deleted, add it to free blocks.
- When a block is written, remove it from free blocks.

> The purpose is to reuse the blocks in which data is previously deleted.

## Implementation

### Read

- Request an array of block indexes to read.
- Read blocks from storage and return data in order.

### Write

- Check data length. And plan to write data in blocks of size `BLOCK_LEN`.
- Search for free blocks(inMEMO) and write data in blocks.
- If no free blocks, extend the file with new blocks.
- Return array of block indexes in the same order as data.

### Delete

- Request an array of block indexes to delete.
- Update block data length to 0.
- Clean block data bytes, by overwriting 0s.(optional)
- Add the block index to free blocks array(inMEMO).

## Optimizations

### Improve read performance with pool of blocks

Read blocks in ascending order of sorted block indexes, can significantly improve read performance and reduce disk wear. (Ascending order as HardDisk only rotates in one direction)

### Improve write performance with pool of blocks

Writing blocks in a uniform direction of sorted block indexes, can significantly improve write performance and reduce disk wear.

## Usage

### Quick Example

```rs
/// Local file path for storage file.
let file_path = "test.dat";

// create new storage
let block_len = 8;
let mut storage = Storage::new(String::from(tmp_file_path), block_len).unwrap();

// write 8 bytes to block 0
let data = [1, 2, 3, 4, 5, 6, 7, 8];
let block_indexes = storage.write(0 as BlockIndex, &data).unwrap();

// read 8 bytes from block 0
let mut read_data = [0; 8];
let block_indexes = storage.read(0 as BlockIndex, &mut data).unwrap();
assert_eq!(read_data, data);

// delete block 0
storage.delete(0 as BlockIndex).unwrap();

// search for free blocks and then write data in blocks
let data = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
let data_chunks = data.chunks(block_len); // [ [1,2,3,4,5,6,7,8], [9,10,11,12,13,14,15,16] ]
let blocks_required = data_chunks.clone().count(); // 2
let allocated_blocks = storage.search_block_allocation_indexes(blocks_required).unwrap();
allocated_blocks.iter().for_each(|block_index| {
    let data_chunk = data_chunks.next().unwrap();
    storage.write(*block_index, &data_chunk).unwrap();
});

// read from allocated_blocks
let mut read_data: Vec<u8> = Vec::new();
allocated_blocks.iter().for_each(|block_index| {
    let mut data_chunk = [0; 8];
    storage.read(*block_index, &mut data_chunk).unwrap();
    read_data.extend_from_slice(&data_chunk);
});
assert_eq!(read_data, vec!(data));
```