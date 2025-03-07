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

pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut head = head;

    let mut before = ListNode::new(0);
    let mut before_tail = &mut before;

    let mut after = ListNode::new(0);
    let mut after_tail = &mut after;

    while let Some(mut node) = head {
        head = node.next.take();
        if node.val < x {
            before_tail.next = Some(node);
            before_tail = before_tail.next.as_mut().unwrap();
        } else {
            after_tail.next = Some(node);
            after_tail = after_tail.next.as_mut().unwrap();
        }
    }

    before_tail.next = after.next.take();

    before.next
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            partition(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 2,
                                next: Some(Box::new(ListNode {
                                    val: 5,
                                    next: Some(Box::new(ListNode::new(2)))
                                }))
                            }))
                        }))
                    }))
                })),
                3
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode {
                                val: 3,
                                next: Some(Box::new(ListNode::new(5)))
                            }))
                        }))
                    }))
                }))
            }))
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            partition(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(1)))
                })),
                2
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode::new(2)))
            }))
        );
    }
}
