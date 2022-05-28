# Logchain

An abstraction build for Storage Engine (`storage` module) to store data as a chain of blocks.

## Idea

### Log

A log is a chain of data **segments**.

Each segment is stored along with `next_segment_index` as the block data of storage. (next_segment_index is -1 if it is the last segment)

To traverse the log from start, the index of **head segment** of the log must be provided.

Although we can traverse the log from any intermediate segment too. So, when we can reduce traversal overhead, we record the block index of the segment closest to point of data we are interested in. (Eg. when we want to append the log from the end, we can record the block index of the tail segment)

### File structure logchain based storage file.

```
|----------------------------|
| BLOCK_LEN        <4 Bytes> |
|----------------------------|
| Log 1's segment 1          | <-- block index 0
|----------------------------|
| Log 2's segment 1          | <-- block index 1
|----------------------------|
| Log 2's segment 2          | <-- block index 2
|----------------------------|
| Log 3's segment 1          | <-- block index 3
|----------------------------|
| Log 4's segment 1          | <-- block index 4
|----------------------------|
| so on...                   |
```

### Structure of block data of a segment

```
|----------------------------------------|-----------------|
| Block 1 dataSize             <4 Bytes> | <- Block header
|----------------------------------------|-----------------|
| Next segment's block index    <4 Bytes>|
|----------------------------------------| <- Block Data
| Segment Data <BLOCK_LEN - dataSize - 4>|
|----------------------------------------|-----------------|
```

### Operations

1. Create a log
2. Append a log
3. Read a log
4. Delete a log

## Usage for xdb

_using mongodb's naming convention to explain_

Logs are requied to implement document storage and index mapping storage.

### Storing documents

The content of a document will be stored in a log.

- Adding document: `create_log` with serialized document as data.
- Reading document: `read_log` with log's head segment index.
- Deleting document: `delete_log` with log's head segment index.
- Updating document: `create_log` with updated serialized document as data and delete_log with old log's head segment index.
