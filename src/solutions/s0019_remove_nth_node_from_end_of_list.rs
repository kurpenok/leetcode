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

fn collect_elements(node: &Option<Box<ListNode>>, elements: &mut Vec<i32>) {
    let mut current = node;

    while let Some(n) = current {
        elements.push(n.val);
        current = &n.next;
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

pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    if n <= 0 {
        return head;
    }

    let n = n as usize;
    let mut elements = Vec::new();
    collect_elements(&head, &mut elements);

    if n > elements.len() {
        return head;
    }

    let index = elements.len() - n;
    elements.remove(index);
    build_list(&elements)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            remove_nth_from_end(
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
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(5)))
                    }))
                }))
            })),
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            remove_nth_from_end(Some(Box::new(ListNode::new(1))), 1),
            None
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            remove_nth_from_end(
                Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode::new(2)))
                })),
                1
            ),
            Some(Box::new(ListNode::new(1)))
        );
    }
}
