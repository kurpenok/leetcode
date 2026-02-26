use std::cell::RefCell;
use std::rc::Rc;

use leetcode::btree::TreeNode;
use leetcode::s0199_binary_tree_right_side_view::right_side_view;

#[test]
fn test_case_1() {
    assert_eq!(
        right_side_view(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
            })))
        })))),
        [1, 3, 4]
    );
}

#[test]
fn test_case_2() {
    assert_eq!(
        right_side_view(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None
                }))),
                right: None
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        })))),
        [1, 3, 4, 5]
    );
}

#[test]
fn test_case_3() {
    assert_eq!(
        right_side_view(Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: None,
            right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
        })))),
        [1, 3]
    );
}

#[test]
fn test_case_4() {
    assert_eq!(right_side_view(None), []);
}
