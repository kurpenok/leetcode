#[cfg(test)]
mod test_s0236 {
    use std::{cell::RefCell, rc::Rc};

    use leetcode::{
        btree::TreeNode, s0236_lowest_common_ancestor_of_a_binary_tree::lowest_common_ancestor,
    };

    #[test]
    fn test_case_1() {
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node7.clone()),
            right: Some(node4.clone()),
        }));
        let node5 = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(node6.clone()),
            right: Some(node2.clone()),
        }));
        let node0 = Rc::new(RefCell::new(TreeNode::new(0)));
        let node8 = Rc::new(RefCell::new(TreeNode::new(8)));
        let node1 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node0.clone()),
            right: Some(node8.clone()),
        }));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(node5.clone()),
            right: Some(node1.clone()),
        }));

        let p = Some(node5.clone());
        let q = Some(node1.clone());
        let expected = Some(root.clone());

        assert_eq!(lowest_common_ancestor(Some(root), p, q), expected);
    }

    #[test]
    fn test_case_2() {
        let node6 = Rc::new(RefCell::new(TreeNode::new(6)));
        let node7 = Rc::new(RefCell::new(TreeNode::new(7)));
        let node4 = Rc::new(RefCell::new(TreeNode::new(4)));
        let node2 = Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(node7.clone()),
            right: Some(node4.clone()),
        }));
        let node5 = Rc::new(RefCell::new(TreeNode {
            val: 5,
            left: Some(node6.clone()),
            right: Some(node2.clone()),
        }));
        let node0 = Rc::new(RefCell::new(TreeNode::new(0)));
        let node8 = Rc::new(RefCell::new(TreeNode::new(8)));
        let node1 = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node0.clone()),
            right: Some(node8.clone()),
        }));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 3,
            left: Some(node5.clone()),
            right: Some(node1.clone()),
        }));

        let p = Some(node5.clone());
        let q = Some(node4.clone());
        let expected = Some(node5.clone());

        assert_eq!(lowest_common_ancestor(Some(root), p, q), expected);
    }

    #[test]
    fn test_case_3() {
        let node2 = Rc::new(RefCell::new(TreeNode::new(2)));
        let root = Rc::new(RefCell::new(TreeNode {
            val: 1,
            left: Some(node2.clone()),
            right: None,
        }));

        let p = Some(root.clone());
        let q = Some(node2.clone());
        let expected = Some(root.clone());

        assert_eq!(lowest_common_ancestor(Some(root), p, q), expected);
    }
}
