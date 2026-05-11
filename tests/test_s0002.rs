#[cfg(test)]
mod test_s0002 {
    use leetcode::list::ListNode;

    use leetcode::s0002_add_two_numbers::add_two_numbers;

    #[test]
    fn test_case_1() {
        assert_eq!(
            add_two_numbers(
                Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode::new(3))),
                    }))
                })),
                Some(Box::new(ListNode {
                    val: 5,
                    next: Some(Box::new(ListNode {
                        val: 6,
                        next: Some(Box::new(ListNode::new(4))),
                    }))
                })),
            ),
            Some(Box::new(ListNode {
                val: 7,
                next: Some(Box::new(ListNode {
                    val: 0,
                    next: Some(Box::new(ListNode::new(8))),
                }))
            }))
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            add_two_numbers(
                Some(Box::new(ListNode::new(0))),
                Some(Box::new(ListNode::new(0))),
            ),
            Some(Box::new(ListNode::new(0)))
        );
    }
}
