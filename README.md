# memdb
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Thread-safe in-memory key-value store. Ideal for development and prototyping.
Does not persist to disk.

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
let mut db = Memdb::open().await?;
db.set("beep", "boop").await?;
let val = db.get("beep").await?;
assert_eq!(val, Some("boop".as_bytes().to_owned()));
Ok(())
```

## Installation
```sh
$ cargo add memdb
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/memdb.svg?style=flat-square
[2]: https://crates.io/crates/memdb
[3]: https://img.shields.io/travis/rustasync/memdb.svg?style=flat-square
[4]: https://travis-ci.com/rustasync/memdb
[5]: https://travis-ci.com/rustasync/memdb.svg?branch=master
[6]: https://crates.io/crates/memdb
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/memdb
