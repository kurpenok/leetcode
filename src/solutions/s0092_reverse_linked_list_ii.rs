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

pub fn reverse_between(
    head: Option<Box<ListNode>>,
    left: i32,
    right: i32,
) -> Option<Box<ListNode>> {
    if head.is_none() {
        return None;
    }

    let mut elements: Vec<i32> = Vec::new();
    collect_elements(&head, &mut elements);

    let mut reversed_kernel: Vec<i32> = elements[left as usize - 1..right as usize].to_vec();
    reversed_kernel.reverse();

    let mut new_elements: Vec<i32> = Vec::new();
    new_elements.extend_from_slice(&elements[..left as usize - 1]);
    new_elements.extend_from_slice(&reversed_kernel);
    new_elements.extend_from_slice(&elements[right as usize..]);

    build_list(&new_elements)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            reverse_between(
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
                2,
                4
            ),
            Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 4,
                    next: Some(Box::new(ListNode {
                        val: 3,
                        next: Some(Box::new(ListNode {
                            val: 2,
                            next: Some(Box::new(ListNode::new(5)))
                        }))
                    }))
                }))
            })),
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            reverse_between(Some(Box::new(ListNode::new(5))), 1, 1),
            Some(Box::new(ListNode::new(5)))
        );
    }
}
