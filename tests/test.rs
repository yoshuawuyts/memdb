extern crate memdb;

use memdb::Memdb;
use std::thread;

#[test]
fn set_get() {
  let mut db = Memdb::default();
  db.set("beep".into(), "boop".into());
  let val = db.get("beep".into());
  assert_eq!(val, Some("boop".to_string()));
}

#[test]
fn threaded_set_get() {
  let db = Memdb::default();

  let mut handle = db.clone();
  let setter = thread::spawn(move || {
    handle.set("beep".into(), "boop".into());

    let handle = db.clone();
    let getter = thread::spawn(move || {
      let val = handle.get("beep".into());
      assert_eq!(val, Some("boop".to_string()));
    });

    getter.join().unwrap();
  });

  setter.join().unwrap();
}
