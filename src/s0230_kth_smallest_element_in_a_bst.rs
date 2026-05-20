use std::cell::RefCell;
use std::rc::Rc;

use crate::btree::TreeNode;

pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut stack = Vec::new();
    let mut current = root;
    let mut count = 0;

    while current.is_some() || !stack.is_empty() {
        while let Some(node) = current {
            stack.push(node.clone());
            current = node.borrow().left.clone();
        }

        let node = stack.pop().unwrap();
        count += 1;

        if count == k {
            return node.borrow().val;
        }

        current = node.borrow().right.clone();
    }

    unreachable!()
}
