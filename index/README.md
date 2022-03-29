# Index Engine

Index is basically an in-memory B-Tree or Hash-Map to search of an item without traversing all items.

## Types of Indexes

1. B-Tree Index: A B-Tree is a self-balancing binary search tree, which also supports range searches.
2. Hash-Map Index: A Hash-Map is a data structure that maps keys to values.

## Optional Features

1. Atomic Locking: No read operation is allowed while any write operation is in progress.

## Persistence of Indexes in storage

Things to ensure while persisting an index in storage:

- Index must be persisted for every write(INSERT/DELETE/UPDATE) operation.
- Any failed operation must be retryable.

### Index Log

Index log file serially stores insert/delete operations, before they are updated in memory.

#### INSERT_LOG tuple format

```
|-------------------------------|
| ENUM_INSERT          <1 Byte> |
|-------------------------------|--------------------|---------
| KEY_LEN             <4 Bytes> | <- Length of key   |
|-------------------------------|                    | <- Key
| KEY_DATA      <KEY_LEN Bytes> | <- Key data        |
|-------------------------------|--------------------|---------
| IND_LEN             <4 Bytes> | <- Index Count     |
|-------------------------------|                    | <- Indexes
| INDEXS    <4 * IND_LEN Bytes> | <- Array of indexs |
|-------------------------------|--------------------|---------
```

#### DELETE_LOG tuple format

```
|-------------------------------|
| ENUM_DELETE          <1 Byte> |
|-------------------------------|--------------------|---------
| KEY_LEN             <4 Bytes> | <- Length of key   |
|-------------------------------|                    | <- Key
| KEY_DATA      <KEY_LEN Bytes> | <- Key data        |
|-------------------------------|--------------------|---------
```

## Flow of Operations

### INSERT

1. Append index log with `INSERT_LOG` tuple.
2. If Atomic Locking is enabled, lock the read operations.
3. Insert new key value pair in index.
4. Unlock the read operations.

### DELETE

1. Append index log with `DELETE_LOG` tuple.
2. If Atomic Locking is enabled, lock the read operations.
3. Delete key from index.
4. Unlock the read operations.

### READ

1. Pool all read operations.
2. Use immutable refrence index for search.

## Required for external implementation

### Channel to appretiate index log

The log can be stored in storage log or in a file.

### Spawning index from existing index log

Read index log from storage and spawn index from it in memory.
