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

fn collect_elements(node: &Option<Box<ListNode>>, elements: &mut Vec<i32>) {
    let mut current = node;

    while let Some(node) = current {
        elements.push(node.val);
        current = &node.next;
    }
}

fn build_list(elements: &[i32]) -> Option<Box<ListNode>> {
    let mut head = None;

    for &val in elements.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = head;
        head = Some(node);
    }

    head
}

pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut elements: Vec<i32> = Vec::new();
    collect_elements(&head, &mut elements);

    let len = elements.len();
    elements.rotate_right(k as usize % len);

    build_list(&elements)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            rotate_right(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 2,
                        next: Some(Box::new(ListNode {
                            val: 3,
                            next: Some(Box::new(ListNode {
                                val: 4,
                                next: Some(Box::new(ListNode::new(5)))
                            }))
                        }))
                    }))
                })),
                2
            ),
            Some(Box::new(ListNode {
                val: 4,
                next: Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(3)))
                        }))
                    }))
                }))
            }))
        )
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            rotate_right(
                Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode {
                        val: 1,
                        next: Some(Box::new(ListNode::new(2)))
                    }))
                })),
                4
            ),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(1)))
                }))
            })),
        )
    }
}
