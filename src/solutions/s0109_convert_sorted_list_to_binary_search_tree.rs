use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

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

fn helper(
    node: Option<&Box<ListNode>>,
    tail: Option<&Box<ListNode>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if node == tail {
        return None;
    }

    let mut slow = node;
    let mut fast = node;

    while fast != tail {
        let fast_next = fast.and_then(|n| n.next.as_ref());
        if fast_next == tail {
            break;
        }

        slow = slow.and_then(|n| n.next.as_ref());
        fast = fast_next.and_then(|n| n.next.as_ref());
    }

    let mut tree_node = TreeNode::new(slow.unwrap().val);
    tree_node.left = helper(node, slow);
    tree_node.right = helper(slow.unwrap().next.as_ref(), tail);

    Some(Rc::new(RefCell::new(tree_node)))
}

pub fn sorted_list_to_bst(head: Option<Box<ListNode>>) -> Option<Rc<RefCell<TreeNode>>> {
    match head {
        Some(head) => helper(Some(&head), None),
        None => None,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            sorted_list_to_bst(Some(Box::new(ListNode {
                val: -10,
                next: Some(Box::new(ListNode {
                    val: -3,
                    next: Some(Box::new(ListNode {
                        val: 0,
                        next: Some(Box::new(ListNode {
                            val: 5,
                            next: Some(Box::new(ListNode::new(9)))
                        }))
                    }))
                }))
            }))),
            Some(Rc::new(RefCell::new(TreeNode {
                val: 0,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: -3,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(-10)))),
                    right: None
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 9,
                    left: Some(Rc::new(RefCell::new(TreeNode::new(5)))),
                    right: None
                })))
            })))
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(sorted_list_to_bst(None), None);
    }
}
