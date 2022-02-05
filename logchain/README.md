# Logchain

An abstraction of build on Storage Engine (`storage` module) to store appendable data in a chain of blocks.

## Idea

### Log

A log is a chain of data **segments**.

Each segment is stored along with `next_segment_index` as the block data of storage. (next_segment_index is -1 if it is the last segment)

In order to traverse the log, index of **head segment** of the log must be provided.

### File structure logchain based storage file.

```
|----------------------------|
| BLOCK_LEN        <4 Bytes> |
|----------------------------|
| Log 1's segment 1          |
|----------------------------|
| Log 2's segment 1          |
|----------------------------|
| Log 2's segment 2          |
|----------------------------|
| Log 3's segment 1          |
|----------------------------|
| Log 4's segment 1          |
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

*using mongodb's naming convention to explain*

Logs are requied to implement document storage and index mapping storage.

### Storing documents

The content of a document will be stored in a log.

- Adding document: `create_log` with serialized document as data.
- Reading document: `read_log` with log's head segment index.
- Deleting document: `delete_log` with log's head segment index.
- Updating document: `create_log` with updated serialized document as data and delete_log with old log's head segment index.

### Main Log (Main indexlog)

Head segment index of all the logs are mapped in the **main** logchain. (Head segment of main logchain is the first block of the storage)

Storing logs with blockindex may not be useful.

Stores head segments of all the logs in the storage.

Index Engine (`index` module) is implemented.

The head segment of main indexlog is the first block of the storage.
