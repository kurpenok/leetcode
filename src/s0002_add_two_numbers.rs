use crate::list::ListNode;

fn add(
    node_1: Option<Box<ListNode>>,
    node_2: Option<Box<ListNode>>,
    carry: i32,
) -> Option<Box<ListNode>> {
    match (node_1, node_2, carry) {
        (None, None, 0) => None,
        (node_1, node_2, carry) => {
            let (value_1, next_1) = match node_1 {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };
            let (value_2, next_2) = match node_2 {
                Some(node) => (node.val, node.next),
                None => (0, None),
            };

            let sum = value_1 + value_2 + carry;

            Some(Box::new(ListNode {
                val: sum % 10,
                next: add(next_1, next_2, sum / 10),
            }))
        }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    add(l1, l2, 0)
}
