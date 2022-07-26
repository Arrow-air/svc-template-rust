//! Hello world example for Rust
//! This is a crate description, needed or else missing_docs warning will occur

use tmp_lib;

#[test]
fn it_add_one() {
    assert_eq!(2, tmp_lib::add_one(1));
}
