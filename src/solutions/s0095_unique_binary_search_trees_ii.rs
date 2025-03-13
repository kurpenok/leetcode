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

fn generate(start: i32, end: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if start > end {
        return vec![None];
    }

    let mut all_trees: Vec<Option<Rc<RefCell<TreeNode>>>> = Vec::new();

    for i in start..=end {
        let left_trees = generate(start, i - 1);
        let right_trees = generate(i + 1, end);

        for l in &left_trees {
            for r in &right_trees {
                let current_tree = Some(Rc::new(RefCell::new(TreeNode::new(i))));
                current_tree.as_ref().unwrap().borrow_mut().left = l.clone();
                current_tree.as_ref().unwrap().borrow_mut().right = r.clone();
                all_trees.push(current_tree);
            }
        }
    }

    all_trees
}

pub fn generate_trees(n: i32) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    if n == 0 {
        vec![]
    } else {
        generate(1, n)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //Some(RefCell { value: TreeNode { val: 3, left: Some(RefCell { value: TreeNode { val: 2, left: Some(RefCell { value: TreeNod
    //e { val: 1, left: None, right: None } }), right: None } }), right: None } })]

    #[test]
    fn test_case_1() {
        assert_eq!(
            generate_trees(3),
            vec![
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                    })))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 1,
                    left: None,
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(2)))),
                        right: None
                    })))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(3))))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
                    }))),
                    right: None
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                        right: None
                    }))),
                    right: None
                })))
            ]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            generate_trees(1),
            vec![Some(Rc::new(RefCell::new(TreeNode::new(1))))]
        );
    }
}
