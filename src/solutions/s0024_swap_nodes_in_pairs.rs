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

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut elements: Vec<i32> = Vec::new();
    collect_elements(&head, &mut elements);
    for i in (0..elements.len()).step_by(2) {
        if i + 1 >= elements.len() {
            break;
        }

        let temp = elements[i];
        elements[i] = elements[i + 1];
        elements[i + 1] = temp;
    }

    build_list(&elements)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            swap_pairs(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode::new(4)))
                    }))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3)))
                    }))
                }))
            }))
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(swap_pairs(None), None);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(
            swap_pairs(Some(Box::new(ListNode::new(1)))),
            Some(Box::new(ListNode::new(1)))
        );
    }

    #[test]
    fn test_case_4() {
        assert_eq!(
            swap_pairs(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            }))),
            Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 1,
                    next: Some(Box::new(ListNode::new(3)))
                }))
            }))
        );
    }
}
