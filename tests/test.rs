#![feature(async_await)]

use memdb::Memdb;
use std::thread;

#[runtime::test]
async fn set_get() {
    let mut db = Memdb::open().await;
    db.set("beep", "boop").await;
    let val = db.get("beep").await;
    assert_eq!(val, Some("boop".as_bytes().to_owned()));
}

#[runtime::test]
async fn threaded_set_get() {
    let db = Memdb::open().await;

    let mut handle = db.clone();
    let setter = runtime::spawn(async {
        handle.set("beep", "boop").await;

        let handle = db.clone();
        runtime::spawn(async {
            let val = handle.get("beep").await;
            assert_eq!(val, Some("boop".as_bytes().to_owned()));
        }).await;
    });
}
