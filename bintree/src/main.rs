extern crate bintree;

use bintree::BinTree;

fn main() {
    println!("Running");
    let mut tree: BinTree<i32> = BinTree::new();
    println!("Root node created as {:#?}", tree);
    assert_eq!(tree.count(), 0);
    tree = BinTree::from(0);
    assert_eq!(tree.count(), 1);
    tree.insert(15);
    tree.insert(17);
    tree.insert(1);
    println!("len now should be 4 => {}", tree.count());
    tree.walk_dfs();
    println!("tree now as {:#?}", tree);
}