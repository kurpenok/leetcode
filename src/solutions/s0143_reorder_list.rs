use std::collections::VecDeque;

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

pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut queue = VecDeque::new();

    let mut node = head.take();
    while let Some(mut n) = node {
        node = n.next.take();
        queue.push_back(n);
    }

    let mut new = ListNode::new(0);
    let mut ptr = &mut new;
    let mut front = true;
    while !queue.is_empty() {
        ptr.next = match front {
            true => queue.pop_front(),
            false => queue.pop_back(),
        };
        front = !front;
        ptr = ptr.next.as_mut().unwrap();
    }

    *head = new.next;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode::new(4))),
                })),
            })),
        }));

        reorder_list(&mut head);

        assert_eq!(
            head,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode::new(3))),
                    })),
                })),
            }))
        );
    }

    #[test]
    fn test_case_2() {
        let mut head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(5))),
                    })),
                })),
            })),
        }));

        reorder_list(&mut head);

        assert_eq!(
            head,
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(3))),
                        })),
                    })),
                })),
            }))
        );
    }
}
