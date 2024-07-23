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

fn check_node(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
    match node {
        Some(node) => {
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            let (left_sum, left_tilt) = check_node(&left);
            let (right_sum, right_tilt) = check_node(&right);

            (
                node.as_ref().borrow().val + left_sum + right_sum,
                (left_sum - right_sum).abs() + left_tilt + right_tilt,
            )
        }
        None => (0, 0),
    }
}

pub fn find_tilt(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let (_, tilt) = check_node(&root);

    tilt
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_tilt(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
            })))),
            1,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_tilt(Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            })))),
            15,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            find_tilt(Some(Rc::new(RefCell::new(TreeNode {
                val: 21,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 14,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            })))),
            9,
        );
    }
}
