use std::collections::VecDeque;

pub struct MinStack {
    minimum: i32,
    stack: VecDeque<i32>,
}

impl MinStack {
    pub fn new() -> Self {
        MinStack {
            minimum: i32::MAX,
            stack: VecDeque::new(),
        }
    }

    pub fn push(&mut self, val: i32) {
        self.minimum = std::cmp::min(self.minimum, val);
        self.stack.push_front(val);
    }

    pub fn pop(&mut self) {
        self.stack.pop_front();
        self.minimum = match self.stack.iter().min() {
            Some(m) => *m,
            None => i32::MAX,
        }
    }

    pub fn top(&self) -> i32 {
        *self.stack.front().unwrap()
    }

    pub fn get_min(&self) -> i32 {
        self.minimum
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut stack = MinStack::new();

        stack.push(-2);
        stack.push(0);
        stack.push(-3);

        assert_eq!(stack.get_min(), -3);

        stack.pop();

        assert_eq!(stack.top(), 0);
        assert_eq!(stack.get_min(), -2);
    }
}
