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

pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut vals_list: Vec<i32> = Vec::new();

    let mut node = head;
    while !node.is_none() {
        let data = node.unwrap();
        vals_list.push(data.val);
        node = data.next;
    }

    let mut index: usize = 0;
    while index <= vals_list.len() / 2 {
        let left = index;
        let right = vals_list.len() - index - 1;

        if vals_list[left] != vals_list[right] {
            return false;
        }

        index += 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let linked_list = Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 2,
                    next: Some(Box::new(ListNode::new(1))),
                })),
            })),
        });

        assert_eq!(is_palindrome(Some(linked_list)), true);
    }

    #[test]
    fn test_case_2() {
        let linked_list = Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode::new(2))),
        });

        assert_eq!(is_palindrome(Some(linked_list)), false);
    }
}
