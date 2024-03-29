#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub next: Option<Box<ListNode>>,
    pub val: i32,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

impl Solution {
    pub fn remove_elements(head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut ptr = &mut head;

        loop {
            match ptr {
                None => break,
                Some(node) if node.val == val => *ptr = node.next.take(),
                Some(node) => ptr = &mut node.next,
            }
        }

        head
    }
}
