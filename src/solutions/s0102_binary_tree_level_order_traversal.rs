use std::cell::RefCell;
use std::collections::HashMap;
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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, values: &mut Vec<(usize, i32)>) {
    match node {
        Some(node) => {
            let val = node.as_ref().borrow().val;
            values.push((depth, val));

            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            helper(left, depth + 1, values);
            helper(right, depth + 1, values);
        }
        None => return,
    }
}

pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut values: Vec<(usize, i32)> = Vec::new();
    helper(root, 0, &mut values);

    let mut values_by_depth: HashMap<usize, Vec<i32>> = HashMap::new();
    for (depth, value) in values {
        values_by_depth.entry(depth).or_insert(vec![]).push(value);
    }

    let mut depth_keys: Vec<usize> = values_by_depth.keys().cloned().collect();
    depth_keys.sort();

    let mut result: Vec<Vec<i32>> = Vec::new();
    for key in depth_keys {
        result.push(values_by_depth[&key].clone());
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            level_order(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7))))
                })))
            })))),
            vec![vec![3], vec![9, 20], vec![15, 7]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            level_order(Some(Rc::new(RefCell::new(TreeNode::new(1))))),
            vec![vec![1]]
        );
    }

    #[test]
    fn test_case_3() {
        let expected_result: Vec<Vec<i32>> = Vec::new();
        assert_eq!(level_order(None), expected_result);
    }
}
