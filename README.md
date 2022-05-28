[![Coverage Status](https://coveralls.io/repos/github/rishavbhowmik/xdb/badge.svg?branch=master)](https://coveralls.io/github/rishavbhowmik/xdb?branch=master)

# What is XDB?

A project starting with an experimental storage engine (Basically fundamental things required to make any kind of Database).
This can be used for:
- KV store obviously
- Graph DB (with its ability to pinpoint the exact location of a node data on a storage file)
- Log datatype - This is like blob, except it can be appended without bothering indexes or its existing payload. So it can store logs or large files (which makes more sense when streamed).

# What's in the Storage engine?

_Will add this doc after renaming the project components. [Intresting story if wondering why this project has weird names](https://github.com/rishavbhowmik/xdb/wiki/Better-Name#story-of-logchain)_

But for now, We have:

1. `storage` - A page store, where a file is segregated into pages (blocks of fixed size). Each page has a page index. This is the primary storage engine.

2. `page-list` (called `logchain` for now!) - A chain of pages that can be used to store data over multiple pages, where each page stores some segment of the data along with page_index of the next page. So this module has abstract functions to do that on top of the storage module.

3. `index` - A module to provide HashMapIndex, UniqueHashMapIndex, BTreeIndex & UniqueBTreeIndex. And CRUD indexes in the storage, with help of the page-list module.

# Desparatly looking for contributors

To be fair, I am still learning Rust and Computer Science in general. I am no pro with databases, just trying to make one (or more if this works).

So If you are a pro and see something good can come out of this project or wish to drive it to something better, please feel free to contribute.

And if you are a Beginner or someone who just wants to learn Rust or how Databases work, I am sure you can learn and experience a lot from this project. This project requires everything, right from using FileSystem, Data Structures, core concepts of Rust, Multithreading, Concurrency, Channeling, Mutexes, Networking, Auth and making tests for all of them, etc.

**Let's have some fun!**
