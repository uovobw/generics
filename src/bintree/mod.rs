use std;

#[derive(Debug)]
pub struct BinTree<T> where T: Ord + std::fmt::Display + Clone {
    root: Option<Box<BinNode<T>>>,
}

impl<T> BinTree<T>
where T: std::fmt::Display + std::cmp::Ord + Clone{
    pub fn new() -> BinTree<T> {
        BinTree {
            root: None,
        }
    }

    pub fn from(value: T) -> BinTree<T> {
        BinTree {
            root: Some(Box::new(
                BinNode::from(value)
            ))
        }
    }

    pub fn count(&self) -> i32 {
        match self.root {
            None => 0,
            Some(ref node) => node.count()
        }
    }

    pub fn walk_dfs(&self) {
        match self.root {
            None => (),
            Some(ref node) => {
                match node.left {
                    None => (),
                    Some(ref left) => left.walk_dfs(),
                }
                match node.right {
                    None => (),
                    Some(ref right) => right.walk_dfs(),
                }
                println!("{}", node)
            },
        }
    }

    pub fn insert(&mut self, value: T) {
        match self.root {
            None => panic!("called insert on empty tree!"),
            Some(ref mut n) => n.insert(value),
        }

    }
}

#[derive(Debug)]
struct BinNode<T>
where T: std::fmt::Display + std::cmp::Ord + Clone{
    value: Option<Box<T>>,
    left: Option<Box<BinNode<T>>>,
    right: Option<Box<BinNode<T>>>,
}

impl<T> std::fmt::Display for BinNode<T>
where T: std::fmt::Display + std::cmp::Ord + Clone{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.value {
            None => write!(f, "empty"),
            Some(ref n) => write!(f, "value: {}", n),
        }
    }
}

impl<T> BinNode<T>
where T: std::fmt::Display + std::cmp::Ord + Clone {
    fn new() -> BinNode<T> {
        BinNode {
            value: None,
            left: None,
            right: None,
        }
    }

    fn from(value: T) -> BinNode<T> {
        BinNode {
            value: Some(Box::new(value)),
            right: None,
            left: None,
        }
    }

    pub fn count(&self) -> i32 {
        let c: i32 = match self.left {
            None => match self.right {
                None => 1,
                Some(ref r) => 1 + r.count()
            },
            Some(ref l) => match self.right {
                None => 1 + l.count(),
                Some(ref this) => 1 + this.count() + self.left.as_ref().unwrap().count()
            }
        };
        c
    }

    fn insert_right(&mut self, value: T) {
        match self.right {
            None => self.right = Some(Box::new(BinNode::from(value))),
            Some(ref mut val) => val.insert(value),
        }
    }

    fn insert_left(&mut self, value: T) {
        match self.left {
            None => self.left = Some(Box::new(BinNode::from(value))),
            Some(ref mut val) => val.insert(value),
        }
    }

    fn set_value(&mut self, value: T) {
        self.value = Some(Box::new(value));
    }

    pub fn insert(&mut self, value: T) {
        match self.value.clone() {
            None => self.set_value(value),
            Some(ref current) => {
                if value.lt(current) {
                    self.insert_right(value);
                } else {
                    self.insert_left(value)
                }
            }
        }
    }

    pub fn walk_dfs(&self) {
        match self.left {
            None => (),
            Some(ref left) => left.walk_dfs(),
        }
        match self.right {
            None => (),
            Some(ref right) => right.walk_dfs(),
        }
        println!("{}", self);
    }
}