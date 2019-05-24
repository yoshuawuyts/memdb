#![feature(async_await)]

use memdb::Memdb;

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
    runtime::spawn(async move {
        handle.set("beep", "boop").await;
        runtime::spawn(async move {
            let handle = handle.clone();
            let val = handle.get("beep").await;
            assert_eq!(val, Some("boop".as_bytes().to_owned()));
        })
        .await;
    })
    .await;
}
