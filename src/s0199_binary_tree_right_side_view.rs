use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

use crate::btree::TreeNode;

pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();

    if root.is_none() {
        return result;
    }

    queue.push_back(root);

    while !queue.is_empty() {
        let depth: usize = queue.len();

        for i in 0..depth {
            if let Some(node) = queue.pop_front().flatten() {
                let node_ref = node.borrow();

                if i == depth - 1 {
                    result.push(node_ref.val);
                }

                if let Some(left) = node_ref.left.clone() {
                    queue.push_back(Some(left));
                }
                if let Some(right) = node_ref.right.clone() {
                    queue.push_back(Some(right));
                }
            }
        }
    }

    result
}
