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

fn match_trees(
    node: &Option<Rc<RefCell<TreeNode>>>,
    sub_node: &Option<Rc<RefCell<TreeNode>>>,
    flag: &mut bool,
    subflag: &mut bool,
) {
    if *flag || *subflag {
        return;
    }

    match node {
        Some(node) => {
            let node_left = node.as_ref().borrow().left.clone();
            let node_right = node.as_ref().borrow().right.clone();

            match sub_node {
                Some(sub_node) => {
                    let sub_node_left = sub_node.as_ref().borrow().left.clone();
                    let sub_node_right = sub_node.as_ref().borrow().right.clone();

                    if node.as_ref().borrow().val != sub_node.as_ref().borrow().val {
                        *subflag = true;
                    }

                    match_trees(&node_left, &sub_node_left, flag, subflag);
                    match_trees(&node_right, &sub_node_right, flag, subflag);
                }
                None => {
                    *subflag = true;
                    return;
                }
            }
        }
        None => {
            if sub_node.is_some() {
                *subflag = true;
            }
        }
    }
}

fn check_node(
    node: &Option<Rc<RefCell<TreeNode>>>,
    sub_root: &Option<Rc<RefCell<TreeNode>>>,
    flag: &mut bool,
) {
    if *flag {
        return;
    }

    match node {
        Some(node) => {
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            let mut subflag: bool = false;
            match_trees(&left, sub_root, flag, &mut subflag);
            if !subflag {
                *flag = true;
            }

            let mut subflag: bool = false;
            match_trees(&right, sub_root, flag, &mut subflag);
            if !subflag {
                *flag = true;
            }

            check_node(&left, sub_root, flag);
            check_node(&right, sub_root, flag);
        }
        None => return,
    }
}

pub fn is_subtree(
    root: Option<Rc<RefCell<TreeNode>>>,
    sub_root: Option<Rc<RefCell<TreeNode>>>,
) -> bool {
    let mut flag: bool = false;

    let mut subflag: bool = false;
    match_trees(&root, &sub_root, &mut flag, &mut subflag);
    if !subflag {
        flag = true;
    }

    check_node(&root, &sub_root, &mut flag);

    flag
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_subtree(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            ),
            true,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_subtree(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                            right: None,
                        }))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                }))),
            ),
            false,
        );
    }
}
