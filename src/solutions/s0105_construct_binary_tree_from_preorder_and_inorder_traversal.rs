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

pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = TreeNode::new(preorder[0]);

    let i = inorder.iter().position(|&x| x == preorder[0])?;
    let inorder_left = inorder[0..i].to_vec();
    let inorder_right = inorder[i + 1..].to_vec();

    let preorder_left = preorder[1..1 + inorder_left.len()].to_vec();
    let preorder_right = preorder[1 + inorder_left.len()..].to_vec();

    if inorder_left.len() > 0 {
        root.left = build_tree(preorder_left, inorder_left);
    }
    if inorder_right.len() > 0 {
        root.right = build_tree(preorder_right, inorder_right);
    }

    Some(Rc::new(RefCell::new(root)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            build_tree(vec![3, 9, 20, 15, 7], vec![9, 3, 15, 20, 7]),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                })))
            })))
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            build_tree(vec![-1], vec![-1]),
            Some(Rc::new(RefCell::new(TreeNode::new(-1))))
        );
    }
}
