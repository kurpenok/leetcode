use std::cell::RefCell;
use std::rc::Rc;

use crate::btree::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    let p_val = p.as_ref().unwrap().borrow().val;
    let q_val = q.as_ref().unwrap().borrow().val;

    let mut current = root;

    while let Some(node) = current {
        let node_val = node.borrow().val;

        if p_val < node_val && q_val < node_val {
            current = node.borrow().left.clone();
        } else if p_val > node_val && q_val > node_val {
            current = node.borrow().right.clone();
        } else {
            return Some(node);
        }
    }

    None
}
