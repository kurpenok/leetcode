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

fn check_node(
    node: &Option<Box<ListNode>>,
    index: &mut usize,
    stop: Option<usize>,
) -> Option<Box<ListNode>> {
    if stop.is_some() && stop.unwrap() == *index {
        return node.clone();
    }

    match node {
        Some(node) => {
            *index += 1;
            check_node(&node.next, index, stop)
        }
        None => None,
    }
}

pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut count = 0;
    check_node(&head, &mut count, None);
    check_node(&head, &mut 0, Some(count / 2))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let node_5 = Some(Box::new(ListNode::new(5)));
        let node_4 = Some(Box::new(ListNode {
            val: 4,
            next: node_5.clone(),
        }));
        let node_3 = Some(Box::new(ListNode {
            val: 3,
            next: node_4.clone(),
        }));
        let node_2 = Some(Box::new(ListNode {
            val: 2,
            next: node_3.clone(),
        }));
        let node_1 = Some(Box::new(ListNode {
            val: 1,
            next: node_2.clone(),
        }));

        assert_eq!(middle_node(node_1), node_3);
    }

    #[test]
    fn test_case_2() {
        let node_6 = Some(Box::new(ListNode::new(6)));
        let node_5 = Some(Box::new(ListNode {
            val: 5,
            next: node_6.clone(),
        }));
        let node_4 = Some(Box::new(ListNode {
            val: 4,
            next: node_5.clone(),
        }));
        let node_3 = Some(Box::new(ListNode {
            val: 3,
            next: node_4.clone(),
        }));
        let node_2 = Some(Box::new(ListNode {
            val: 2,
            next: node_3.clone(),
        }));
        let node_1 = Some(Box::new(ListNode {
            val: 1,
            next: node_2.clone(),
        }));

        assert_eq!(middle_node(node_1), node_4);
    }
}
