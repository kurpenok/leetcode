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

fn check_node(node: Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
    match node {
        Some(node) => {
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            values.push(node.as_ref().borrow().val);

            check_node(left, values);
            check_node(right, values);
        }
        None => return,
    }
}

pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut values: Vec<i32> = Vec::new();
    check_node(root, &mut values);
    values.sort();

    let mut minimal_difference: i32 = i32::MAX;
    for i in 1..values.len() {
        minimal_difference = std::cmp::min(minimal_difference, values[i] - values[i - 1]);
    }

    minimal_difference
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            get_minimum_difference(Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            })))),
            1,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            get_minimum_difference(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 48,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(12)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(49)))),
                }))),
            })))),
            1,
        );
    }
}
