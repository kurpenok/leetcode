use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

fn is_same_tree(s: &Option<Rc<RefCell<TreeNode>>>, t: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (s, t) {
        (Some(s), Some(t)) => {
            let s_borrow = s.borrow();
            let t_borrow = t.borrow();
            s_borrow.val == t_borrow.val
                && is_same_tree(&s_borrow.left, &t_borrow.left)
                && is_same_tree(&s_borrow.right, &t_borrow.right)
        }
        (None, None) => true,
        _ => false,
    }
}

fn check_subtree(
    node: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    match node {
        Some(n) => {
            let n_borrow = n.borrow();
            is_same_tree(node, sub_root)
                || check_subtree(&n_borrow.left, sub_root)
                || check_subtree(&n_borrow.right, sub_root)
        }
        None => false,
    }
}

pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    check_subtree(&root, &sub_root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_subtree(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            ),
            true,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_subtree(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            ),
            false,
        );
    }
}
