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

fn check_node(node: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match node {
        Some(node) => {
            let node_val = node.as_ref().borrow().val;
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            if node_val == val {
                return Some(node);
            }

            let left_subtree = check_node(left, val);
            let right_subtree = check_node(right, val);

            if left_subtree.is_some() {
                return left_subtree;
            } else if right_subtree.is_some() {
                return right_subtree;
            }

            None
        }
        None => None,
    }
}

pub fn search_bst(root: Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    check_node(root, val)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            search_bst(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                }))),
                2
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            }))),
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            search_bst(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                }))),
                5
            ),
            None
        )
    }
}
