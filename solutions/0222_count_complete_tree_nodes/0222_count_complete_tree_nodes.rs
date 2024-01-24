use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Eq, PartialEq)]
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

impl Solution {
    fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        const COUNT: fn(Option<Rc<RefCell<TreeNode>>>) -> i32 = |root| {
            if let Some(root) = root {
                COUNT(root.borrow().left.clone()) + COUNT(root.borrow().right.clone()) + 1
            } else {
                0
            }
        };

        COUNT(root)
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        dfs(root)
    }
}
