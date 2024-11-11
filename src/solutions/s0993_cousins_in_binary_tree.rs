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

#[derive(Debug)]
pub struct TreeNodeRelation {
    pub parent_value: i32,
    pub depth: usize,
}

fn dfs(
    node: Option<Rc<RefCell<TreeNode>>>,
    parent_value: i32,
    depth: usize,
    nodes: &mut HashMap<i32, TreeNodeRelation>,
) {
    match node {
        Some(node) => {
            let value = node.as_ref().borrow().val;
            let left = node.as_ref().borrow().left.clone();
            let right = node.as_ref().borrow().right.clone();

            nodes.insert(
                value,
                TreeNodeRelation {
                    parent_value,
                    depth,
                },
            );

            dfs(left, value, depth + 1, nodes);
            dfs(right, value, depth + 1, nodes);
        }
        None => return,
    }
}

pub fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let mut nodes: HashMap<i32, TreeNodeRelation> = HashMap::new();

    dfs(root, -1, 0, &mut nodes);

    nodes[&x].parent_value != nodes[&y].parent_value && nodes[&x].depth == nodes[&y].depth
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            is_cousins(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                        right: None,
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                4,
                3,
            ),
            false,
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            is_cousins(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    }))),
                }))),
                5,
                4,
            ),
            true,
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            is_cousins(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4)))),
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                }))),
                2,
                3,
            ),
            false,
        );
    }
}
