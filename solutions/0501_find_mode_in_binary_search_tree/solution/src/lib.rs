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

fn check_node(node: Option<Rc<RefCell<TreeNode>>>, values: &mut HashMap<i32, usize>) {
    match node {
        Some(node) => {
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            values
                .entry(node.as_ref().borrow().val)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            check_node(left, values);
            check_node(right, values);
        }
        None => return,
    }
}

pub fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut values: HashMap<i32, usize> = HashMap::new();

    check_node(root, &mut values);

    let max_value: usize = *values.values().max().unwrap();
    values
        .into_iter()
        .filter(|&(_, v)| v == max_value)
        .map(|(k, _)| k)
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            find_mode(Some(Rc::new(RefCell::new(TreeNode {
                val: 1,
                left: None,
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                    right: None,
                })))
            })))),
            vec![2]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            find_mode(Some(Rc::new(RefCell::new(TreeNode::new(0))))),
            vec![0]
        );
    }
}
