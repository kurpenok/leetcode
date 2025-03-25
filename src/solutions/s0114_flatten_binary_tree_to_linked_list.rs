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

pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = std::iter::once(root.clone())
        .flatten()
        .collect::<Vec<Rc<RefCell<TreeNode>>>>();

    let mut head = Rc::new(RefCell::new(TreeNode::new(0)));

    while let Some(node) = stack.pop() {
        let mut node_ref = node.borrow_mut();

        if let Some(right) = node_ref.right.take() {
            stack.push(right);
        }

        if let Some(left) = node_ref.left.take() {
            stack.push(left);
        }

        drop(node_ref);

        head.borrow_mut().right = Some(node.clone());
        head = node;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 5,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(6)))),
            }))),
        })));

        flatten(&mut root);

        assert_eq!(
            root,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode {
                                val: 5,
                                left: None,
                                right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
                            })))
                        })))
                    })))
                })))
            })))
        );
    }

    #[test]
    fn test_case_2() {
        let mut root: Option<Rc<RefCell<TreeNode>>> = None;

        flatten(&mut root);

        assert_eq!(root, None);
    }

    #[test]
    fn test_case_3() {
        let mut root: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode::new(0))));

        flatten(&mut root);

        assert_eq!(root, Some(Rc::new(RefCell::new(TreeNode::new(0)))));
    }
}
