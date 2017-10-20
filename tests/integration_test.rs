extern crate bintree;

mod common;

#[test]
fn new_tree_has_zero_len() {
    let tree = common::create_tree::<i32>();
    assert!(tree.count() == 0, "Newly created tree had non-zero size");
}

