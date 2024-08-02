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

fn check_node(
    node: Option<Rc<RefCell<TreeNode>>>,
    level: usize,
    level_elements: &mut HashMap<usize, Vec<i64>>,
) {
    match node {
        Some(node) => {
            let val = node.as_ref().borrow().val;
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            level_elements
                .entry(level)
                .and_modify(|elements| elements.push(val as i64))
                .or_insert(vec![val as i64]);

            check_node(left, level + 1, level_elements);
            check_node(right, level + 1, level_elements);
        }
        None => return,
    }
}

pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut level_elements: HashMap<usize, Vec<i64>> = HashMap::new();
    check_node(root, 0, &mut level_elements);

    let mut levels: Vec<_> = level_elements.keys().collect();
    levels.sort();

    let mut averages: Vec<f64> = Vec::new();
    for level in levels {
        averages.push(
            level_elements[level].iter().sum::<i64>() as f64 / level_elements[level].len() as f64,
        );
    }

    averages
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode::new(9)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 20,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
            })))),
            vec![3.0, 14.5, 11.0],
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(15)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(20)))),
            })))),
            vec![3.0, 14.5, 11.0],
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            average_of_levels(Some(Rc::new(RefCell::new(TreeNode {
                val: 2147483647,
                left: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
                right: Some(Rc::new(RefCell::new(TreeNode::new(2147483647)))),
            })))),
            vec![2147483647.0, 2147483647.0],
        );
    }
}
