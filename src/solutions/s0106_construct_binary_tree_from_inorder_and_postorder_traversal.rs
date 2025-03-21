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

pub fn build_tree(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if postorder.is_empty() {
        return None;
    }

    let root_val = *postorder.last().unwrap();
    let mut root = TreeNode::new(root_val);
    let pos = inorder.iter().position(|&x| x == root_val).unwrap();

    let right_inorder = &inorder[pos + 1..];
    let right_postorder = &postorder[pos..postorder.len() - 1];
    root.right = build_tree(right_inorder.to_vec(), right_postorder.to_vec());

    let left_inorder = &inorder[..pos];
    let left_postorder = &postorder[..pos];
    root.left = build_tree(left_inorder.to_vec(), left_postorder.to_vec());

    Some(Rc::new(RefCell::new(root)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            build_tree(vec![9, 3, 15, 20, 7], vec![9, 15, 7, 20, 3]),
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
