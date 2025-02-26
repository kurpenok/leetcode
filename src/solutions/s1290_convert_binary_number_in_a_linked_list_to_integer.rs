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

    while let Some(n) = current {
        elements.push(n.val);
        current = &n.next;
    }
}

pub fn get_decimal_value(head: Option<Box<ListNode>>) -> i32 {
    let mut elements: Vec<i32> = Vec::new();
    collect_elements(&head, &mut elements);

    let mut decimal_value: i32 = 0;
    for i in 0..elements.len() {
        if elements[i] == 1 {
            decimal_value += 2i32.pow((elements.len() - i - 1) as u32);
        }
    }

    decimal_value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            get_decimal_value(Some(Box::new(ListNode {
                val: 1,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(1)))
                }))
            }))),
            5
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(get_decimal_value(Some(Box::new(ListNode::new(0)))), 0);
    }
}
