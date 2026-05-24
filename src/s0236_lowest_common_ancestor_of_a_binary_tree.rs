use std::{cell::RefCell, rc::Rc};

use crate::btree::TreeNode;

pub fn lowest_common_ancestor(
    root: Option<Rc<RefCell<TreeNode>>>,
    p: Option<Rc<RefCell<TreeNode>>>,
    q: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        None => None,
        Some(node) => {
            if let Some(p_node) = &p {
                if Rc::ptr_eq(&node, p_node) {
                    return Some(node);
                }
            }
            if let Some(q_node) = &q {
                if Rc::ptr_eq(&node, q_node) {
                    return Some(node);
                }
            }

            let left = lowest_common_ancestor(node.borrow().left.clone(), p.clone(), q.clone());
            let right = lowest_common_ancestor(node.borrow().right.clone(), p.clone(), q.clone());

            match (left, right) {
                (Some(_), Some(_)) => Some(node),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                (None, None) => None,
            }
        }
    }
}
