# memdb
[![crates.io version][1]][2] [![build status][3]][4]
[![downloads][5]][6] [![docs.rs docs][7]][8]

Thread-safe in-memory key-value store. Ideal for development and prototyping.
Does not persist to disk.

- [Documentation][8]
- [Crates.io][2]

## Usage
```rust
extern crate memdb;

use memdb::Memdb;

let mut db = Memdb::default();
db.set("beep".into(), "boop".into());

let val = db.get("beep".into());
assert_eq!(val, Some("boop".to_string()));
```

## Installation
```sh
$ cargo add memdb
```

## License
[MIT](./LICENSE-MIT) OR [Apache-2.0](./LICENSE-APACHE)

[1]: https://img.shields.io/crates/v/memdb.svg?style=flat-square
[2]: https://crates.io/crates/memdb
[3]: https://img.shields.io/travis/rust-net-web/memdb.svg?style=flat-square
[4]: https://travis-ci.org/rust-net-web/memdb
[5]: https://img.shields.io/crates/d/memdb.svg?style=flat-square
[6]: https://crates.io/crates/memdb
[7]: https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square
[8]: https://docs.rs/memdb
