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

pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node) = root {
        let left = node.as_ref().borrow().left.clone();
        let right = node.as_ref().borrow().right.clone();

        node.as_ref().borrow_mut().left = invert_tree(right);
        node.as_ref().borrow_mut().right = invert_tree(left);

        return Some(node);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut root = TreeNode::new(2);
        root.left = Some(Rc::new(RefCell::new(TreeNode::new(1))));
        root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));

        let mut expected = TreeNode::new(2);
        expected.left = Some(Rc::new(RefCell::new(TreeNode::new(3))));
        expected.right = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(
            invert_tree(Some(Rc::new(RefCell::new(root)))),
            Some(Rc::new(RefCell::new(expected)))
        );
    }
}
