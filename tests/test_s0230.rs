#[cfg(test)]
mod test_s0230 {
    use std::cell::RefCell;
    use std::rc::Rc;

    use leetcode::{btree::TreeNode, s0230_kth_smallest_element_in_a_bst::kth_smallest};

    #[test]
    fn test_case_1() {
        assert_eq!(
            kth_smallest(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 3,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: Some(Rc::new(RefCell::new(TreeNode::new(2))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                }))),
                1
            ),
            1
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            kth_smallest(
                Some(Rc::new(RefCell::new(TreeNode {
                    val: 5,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 3,
                        left: Some(Rc::new(RefCell::new(TreeNode {
                            val: 2,
                            left: None,
                            right: Some(Rc::new(RefCell::new(TreeNode::new(1))))
                        }))),
                        right: Some(Rc::new(RefCell::new(TreeNode::new(4))))
                    }))),
                    right: Some(Rc::new(RefCell::new(TreeNode::new(6))))
                }))),
                3
            ),
            3
        );
    }
}
