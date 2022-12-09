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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        fn helper(node: &Option<Box<ListNode>>, values: &mut Vec<i32>) {
            match node {
                Some(node) => {
                    helper(&node.next, values);
                    values.push(node.val);
                },
                None => { return; }
            }
        }

        let mut values: Vec<i32> = Vec::new();
        helper(&head, &mut values);

        let mut head = head;
        let mut ptr = &mut head;
        let mut i: usize = 0;

        loop {
            match ptr {
                Some(node) => {
                    node.val = values[i];
                    ptr = &mut node.next;
                },
                None => break,
            }
            i += 1;
        }

        head
    }
}
