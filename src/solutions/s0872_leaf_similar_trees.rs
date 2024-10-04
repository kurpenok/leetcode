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

fn get_leafs(node: Option<Rc<RefCell<TreeNode>>>, leafs: &mut Vec<i32>) {
    match node {
        Some(node) => {
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            if left.is_none() && right.is_none() {
                leafs.push(node.as_ref().borrow().val);
            }

            get_leafs(left, leafs);
            get_leafs(right, leafs);
        }
        None => return,
    }
}

pub fn leaf_similar(
    root1: Option<Rc<RefCell<TreeNode>>>,
    root2: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut root1_leafs = Vec::new();
    let mut root2_leafs = Vec::new();

    get_leafs(root1, &mut root1_leafs);
    get_leafs(root2, &mut root2_leafs);

    root1_leafs == root2_leafs
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            leaf_similar(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(8))))
                    })))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 5,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(8))))
                        })))
                    })))
                })))
            ),
            true
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            leaf_similar(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
                })))
            ),
            false
        )
    }
}
