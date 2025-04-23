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

pub struct BSTIterator {
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl BSTIterator {
    pub fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = BSTIterator { stack: Vec::new() };
        iter.push_left(root);
        iter
    }

    pub fn next(&mut self) -> i32 {
        let node = self.stack.pop().unwrap();

        let val = node.borrow().val;
        if let Some(right) = node.borrow().right.clone() {
            self.push_left(Some(right));
        }

        val
    }

    pub fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn push_left(&mut self, mut node: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(n) = node {
            self.stack.push(n.clone());
            node = n.borrow().left.clone();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut bst_iterator: BSTIterator =
            BSTIterator::new(Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 15,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
                }))),
            }))));

        assert_eq!(bst_iterator.next(), 3);
        assert_eq!(bst_iterator.next(), 7);
        assert_eq!(bst_iterator.has_next(), true);
        assert_eq!(bst_iterator.next(), 9);
        assert_eq!(bst_iterator.has_next(), true);
        assert_eq!(bst_iterator.next(), 15);
        assert_eq!(bst_iterator.has_next(), true);
        assert_eq!(bst_iterator.next(), 20);
        assert_eq!(bst_iterator.has_next(), false);
    }
}
