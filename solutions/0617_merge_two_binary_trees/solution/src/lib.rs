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

fn check_node(
    node1: Option<Rc<RefCell<TreeNode>>>,
    node2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if node1.is_none() && node2.is_none() {
        return None;
    }

    let mut node: Option<Rc<RefCell<TreeNode>>> = None;

    if node1.is_some() && node2.is_some() {
        let node1 = node1.unwrap();
        let node2 = node2.unwrap();

        let node1_val = node1.as_ref().borrow().val;
        let node1_left = node1.as_ref().borrow().left.clone();
        let node1_right = node1.as_ref().borrow().right.clone();

        let node2_val = node2.as_ref().borrow().val;
        let node2_left = node2.as_ref().borrow().left.clone();
        let node2_right = node2.as_ref().borrow().right.clone();

        node = Some(Rc::new(RefCell::new(TreeNode {
            val: node1_val + node2_val,
            left: check_node(node1_left, node2_left),
            right: check_node(node1_right, node2_right),
        })));
    } else if node1.is_some() && node2.is_none() {
        let node1 = node1.unwrap();

        let node1_val = node1.as_ref().borrow().val;
        let node1_left = node1.as_ref().borrow().left.clone();
        let node1_right = node1.as_ref().borrow().right.clone();

        node = Some(Rc::new(RefCell::new(TreeNode {
            val: node1_val,
            left: check_node(node1_left, None),
            right: check_node(node1_right, None),
        })));
    } else if node1.is_none() && node2.is_some() {
        let node2 = node2.unwrap();

        let node2_val = node2.as_ref().borrow().val;
        let node2_left = node2.as_ref().borrow().left.clone();
        let node2_right = node2.as_ref().borrow().right.clone();

        node = Some(Rc::new(RefCell::new(TreeNode {
            val: node2_val,
            left: check_node(None, node2_left),
            right: check_node(None, node2_right),
        })));
    }

    node
}

pub fn merge_trees(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    check_node(root1, root2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            merge_trees(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    }))),
                }))),
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            }))),
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            merge_trees(
                Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: None,
                })))
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
        );
    }
}
