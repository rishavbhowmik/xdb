# Storage Engine

## Idea

### Storage

Mantain a storage file, segmented in blocks of size fixed `BLOCK_LEN`.
Each block stores data length and data against an index.
Blocks of data_length 0 can be reused.
If all blocks are used, the file is extended with new blocks.

### Read

Request array of block indexes to read.
Read blocks from storage and return data in order.
The Data can be returned as stream or as pipe.

### Write

Check data length. And plan to write data in blocks of size `BLOCK_LEN`.
Search for free blocks(inMEMO) and write data in blocks.
If no free blocks, extend file with new blocks.
Return array of block indexes.

### Delete

Request array of block indexes to delete.
Update block data length to 0.
Clean block data with 0.(optional)
Add block to free blocks(inMEMO).

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

- Initialize free blocks with all blocks in file with data_length 0.
- When a block is deleted, add it to free blocks.
- When a block is written, remove it from free blocks.

## Optimizations

### Improve read performance with pool of blocks

Read blocks in assending order of sorted block indexes, can significantly improve read performance and reduce disk wear. (Assending order as HardDisk only rotates in one direction)

### Improve write performance with pool of blocks

Write blocks in uniform direction of sorted block indexes, can significantly improve write performance and reduce disk wear.
