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

fn get_paths(node: Option<Rc<RefCell<TreeNode>>>, paths: &mut Vec<String>, path: String) {
    match node {
        Some(node) => {
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            let mut path = String::from(path);
            if path != "" {
                path.push_str("->");
            }
            path.push_str(&node.as_ref().borrow().val.to_string());

            if left.is_none() && right.is_none() {
                paths.push(path);
                return;
            }

            get_paths(left, paths, path.clone());
            get_paths(right, paths, path.clone());
        }
        None => return,
    }
}

pub fn binary_tree_paths(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<String> {
    let mut paths: Vec<String> = Vec::new();

    get_paths(root, &mut paths, "".to_string());

    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let root = Some(Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
        })));

        assert_eq!(
            binary_tree_paths(root),
            vec!["1->2->5".to_string(), "1->3".to_string()]
        );
    }

    #[test]
    fn test_case_2() {
        let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));

        assert_eq!(binary_tree_paths(root), vec!["1".to_string()]);
    }
}
