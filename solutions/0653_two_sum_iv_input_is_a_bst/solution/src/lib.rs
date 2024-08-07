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

fn check_node(node: Option<Rc<RefCell<TreeNode>>>, elements: &mut Vec<i32>) {
    match node {
        Some(node) => {
            elements.push(node.as_ref().borrow().val);

            check_node(node.as_ref().borrow().left.clone(), elements);
            check_node(node.as_ref().borrow().right.clone(), elements);
        }
        None => return,
    }
}

pub fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    let mut elements: Vec<i32> = Vec::new();

    check_node(root, &mut elements);

    for i in 0..elements.len() {
        for j in i + 1..elements.len() {
            if elements[i] + elements[j] == k {
                return true;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_target(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    }))),
                }))),
                9,
            ),
            true,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_target(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 6,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    }))),
                }))),
                28,
            ),
            false,
        );
    }
}
