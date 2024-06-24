use std::cell::RefCell;
use std::collections::VecDeque;
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

pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut sum: i32 = 0;

    let mut queue: VecDeque<Option<Rc<RefCell<TreeNode>>>> = VecDeque::new();
    queue.push_back(root);

    while !queue.is_empty() {
        let node = queue.pop_front().unwrap();

        match node {
            Some(node) => {
                let left = node.as_ref().borrow().left.clone();
                let right = node.as_ref().borrow().right.clone();

                if left.is_some() {
                    let left_children = left.clone().unwrap().as_ref().borrow().left.clone();
                    let right_children = left.clone().unwrap().as_ref().borrow().right.clone();
                    if left_children.is_none() && right_children.is_none() {
                        sum += left.clone().unwrap().as_ref().borrow().val;
                    }
                }

                queue.push_back(left);
                queue.push_back(right);
            }
            None => continue,
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 20,
                left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
            }))),
        })));

        assert_eq!(sum_of_left_leaves(tree), 24);
    }

    #[test]
    fn test_case_2() {
        let tree = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(sum_of_left_leaves(tree), 0);
    }

    #[test]
    fn test_case_3() {
        let tree = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(sum_of_left_leaves(tree), 4);
    }
}
