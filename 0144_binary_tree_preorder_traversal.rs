use std::rc::Rc;
use std::cell::RefCell;

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
            right: None
        }
    }
}

impl Solution {
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        fn recursive_helper(node: &Option<Rc<RefCell<TreeNode>>>, values: &mut Vec<i32>) {
            if let Some(v) = node {
                let v = v.borrow();

                values.push(v.val);
                recursive_helper(&v.left, values);
                recursive_helper(&v.right, values);
            }
        }

        let mut values: Vec<i32> = Vec::new();

        if let Some(v) = root {
            recursive_helper(&Some(v), &mut values);
        }

        values
    }
}
