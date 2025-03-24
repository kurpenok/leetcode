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

fn collect_paths(
    node: &Option<Rc<RefCell<TreeNode>>>,
    target_sum: i32,
    path: &mut Vec<i32>,
    paths: &mut Vec<Vec<i32>>,
) {
    if let Some(n) = node {
        let val = n.borrow().val;
        let left = &n.borrow().left;
        let right = &n.borrow().right;

        path.push(val);

        if target_sum == val && left.is_none() && right.is_none() {
            paths.push(path.clone());
        }

        collect_paths(left, target_sum - val, path, paths);
        collect_paths(right, target_sum - val, path, paths);

        path.pop();
    }
}

pub fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> Vec<Vec<i32>> {
    let mut paths: Vec<Vec<i32>> = Vec::new();

    collect_paths(&root, target_sum, &mut vec![], &mut paths);

    paths
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            path_sum(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 11,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
                        }))),
                        right: None
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(13)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                        })))
                    })))
                }))),
                22
            ),
            vec![vec![5, 4, 11, 2], vec![5, 8, 4, 5]]
        );
    }

    #[test]
    fn test_case_2() {
        let expected_result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(
            path_sum(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                }))),
                5
            ),
            expected_result
        );
    }

    #[test]
    fn test_case_3() {
        let expected_result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(
            path_sum(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: None
                }))),
                0
            ),
            expected_result
        );
    }
}
