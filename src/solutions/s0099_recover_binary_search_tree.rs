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

fn in_order_traversal(
    node: &Option<Rc<RefCell<TreeNode>>>,
    prev: &mut Option<Rc<RefCell<TreeNode>>>,
    first: &mut Option<Rc<RefCell<TreeNode>>>,
    second: &mut Option<Rc<RefCell<TreeNode>>>,
) {
    if let Some(n) = node {
        let n_ref = n.borrow();
        in_order_traversal(&n_ref.left, prev, first, second);

        if let Some(prev_node) = prev {
            let prev_val = prev_node.borrow().val;
            let current_val = n_ref.val;
            if prev_val > current_val {
                if first.is_none() {
                    *first = Some(prev_node.clone());
                }
                *second = Some(n.clone());
            }
        }
        *prev = Some(n.clone());

        in_order_traversal(&n_ref.right, prev, first, second);
    }
}

pub fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut prev = None;
    let mut first = None;
    let mut second = None;

    in_order_traversal(root, &mut prev, &mut first, &mut second);

    if let (Some(first_node), Some(second_node)) = (first, second) {
        let mut first_borrow = first_node.borrow_mut();
        let mut second_borrow = second_node.borrow_mut();
        std::mem::swap(&mut first_borrow.val, &mut second_borrow.val);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
            }))),
            right: None,
        })));

        recover_tree(&mut tree);

        assert_eq!(
            tree,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
                right: None,
            })))
        );
    }

    #[test]
    fn test_case_2() {
        let mut tree: Option<Rc<RefCell<TreeNode>>> = Some(Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: None,
            }))),
        })));

        recover_tree(&mut tree);

        assert_eq!(
            tree,
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: None
                })))
            })))
        );
    }
}
