#[cfg(test)]
mod test_s0235 {
    use std::{cell::RefCell, rc::Rc};

    use leetcode::{
        btree::TreeNode,
        s0235_lowest_common_ancestor_of_a_binary_search_tree::lowest_common_ancestor,
    };

    #[test]
    fn test_case_1() {
        assert_eq!(
            lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(9))))
                    })))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                    })))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(9))))
                })))
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 6,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                    })))
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 8,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(9))))
                })))
            })))
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 6,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 2,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                        right: Some(Rc::new(RefCell::new(TreeNode {
                            val: 4,
                            left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                            right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                        })))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 8,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(7)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(9))))
                    })))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                    right: Some(Rc::new(RefCell::new(TreeNode {
                        val: 4,
                        left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                    })))
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                })))
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(0)))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(3)))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(5))))
                })))
            })))
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            lowest_common_ancestor(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None
                }))),
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                    right: None
                }))),
                Some(Rc::new(RefCell::new(TreeNode::new(1))))
            ),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 2,
                left: Some(Rc::new(RefCell::new(TreeNode::new(1)))),
                right: None
            })))
        );
    }
}
