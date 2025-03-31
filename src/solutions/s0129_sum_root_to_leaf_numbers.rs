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

fn collect_paths(node: Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<i32>, path: String) {
    match node {
        Some(node) => {
            let left = node.borrow().left.clone();
            let right = node.borrow().right.clone();

            let mut path = String::from(path);
            path.push_str(&node.borrow().val.to_string());

            if left.is_none() && right.is_none() {
                paths.push(path.parse::<i32>().unwrap());
                return;
            }

            collect_paths(left, paths, path.clone());
            collect_paths(right, paths, path.clone());
        }
        None => return,
    }
}

pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut paths: Vec<i32> = Vec::new();

    collect_paths(root, &mut paths, String::new());

    paths.iter().sum::<i32>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            sum_numbers(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
            })))),
            25
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            sum_numbers(Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(0))))
            })))),
            1026
        );
    }
}
