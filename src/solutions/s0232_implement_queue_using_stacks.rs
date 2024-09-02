#[derive(Debug, PartialEq, Eq)]
pub struct MyQueue {
    queue: Vec<i32>,
}

impl MyQueue {
    pub fn new() -> Self {
        MyQueue { queue: Vec::new() }
    }

    pub fn push(&mut self, n: i32) {
        self.queue.push(n)
    }

    pub fn pop(&mut self) -> i32 {
        let first = self.queue[0];
        self.queue.remove(0);
        first
    }

    pub fn peek(&self) -> i32 {
        self.queue[0]
    }

    pub fn empty(&self) -> bool {
        self.queue.len() == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut queue = MyQueue::new();

        queue.push(1);
        assert_eq!(MyQueue { queue: vec![1] }, queue);

        queue.push(2);
        assert_eq!(MyQueue { queue: vec![1, 2] }, queue);

        assert_eq!(queue.peek(), 1);

        assert_eq!(queue.pop(), 1);
        assert_eq!(MyQueue { queue: vec![2] }, queue);

        assert_eq!(queue.empty(), false);
    }
}
