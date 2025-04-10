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

pub fn insertion_sort_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        None => return None,
        Some(node) => {
            let mut unsorted = Some(node);
            let mut sorted = ListNode::new(std::i32::MIN);

            while let Some(mut node_to_insert) = unsorted {
                unsorted = node_to_insert.next.take();

                let mut sorted_ref = &mut sorted;
                while sorted_ref.next.is_some()
                    && sorted_ref.next.as_ref().unwrap().val < node_to_insert.val
                {
                    sorted_ref = sorted_ref.next.as_mut().unwrap()
                }

                node_to_insert.next = sorted_ref.next.take();
                sorted_ref.next = Some(node_to_insert);
            }

            sorted.next
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            insertion_sort_list(Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode::new(3)))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4)))
                    }))
                }))
            }))
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            insertion_sort_list(Some(Box::new(ListNode {
                val: -1,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(0)))
                        }))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: -1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 4,
                            next: Some(Box::new(ListNode::new(5)))
                        }))
                    }))
                }))
            }))
        );
    }
}
