#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

fn helper(
    node_1: Option<Box<ListNode>>,
    node_2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    if node_1.is_none() && node_2.is_none() && carry == 0 {
        return None;
    }

    let node_1 = node_1.unwrap_or(Box::new(ListNode::new(0)));
    let node_2 = node_2.unwrap_or(Box::new(ListNode::new(0)));

    let sum = node_1.val + node_2.val + carry;
    let mut current_node = ListNode::new(sum % 10);
    current_node.next = helper(node_1.next, node_2.next, sum / 10);

    Some(Box::new(current_node))
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    helper(l1, l2, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode::new(4))),
                    }))
                })),
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8))),
                }))
            }))
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            add_two_numbers(
                Some(Box::new(ListNode::new(0))),
                Some(Box::new(ListNode::new(0))),
            ),
            Some(Box::new(ListNode::new(0)))
        );
    }
}
