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

fn check_node(node: Option<Rc<RefCell<TreeNode>>>, path: &str, paths: &mut Vec<String>) {
    match node {
        Some(node) => {
            let current_path = format!("{}{}", path, node.borrow().val).to_string();

            if node.borrow_mut().left.is_none() && node.borrow_mut().right.is_none() {
                paths.push(current_path);
                return;
            }

            check_node(node.borrow_mut().left.take(), &current_path, paths);
            check_node(node.borrow_mut().right.take(), &current_path, paths);
        }
        None => return,
    }
}

pub fn sum_root_to_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut paths: Vec<String> = Vec::new();
    check_node(root, "", &mut paths);

    let mut sum: i32 = 0;
    for path in paths {
        sum += i32::from_str_radix(&path, 2).unwrap();
    }

    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 0,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                }))),
            })))),
            22,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            sum_root_to_leaf(Some(Rc::new(RefCell::new(TreeNode::new(0))))),
            0,
        );
    }
}
