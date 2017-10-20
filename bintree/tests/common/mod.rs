extern crate bintree;

use bintree::BinTree;
use std::fmt;

pub fn create_tree<G: Ord + fmt::Display + Copy>() -> BinTree<G> {
    BinTree::<G>::new()
}