use std::collections::VecDeque;

#[derive(Debug)]
struct MyStack {
    data: VecDeque<i32>,
}

impl MyStack {
    fn new() -> Self {
        MyStack {
            data: VecDeque::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.data.push_front(x);
    }

    fn pop(&mut self) -> i32 {
        self.data.pop_front().unwrap()
    }

    fn top(&self) -> i32 {
        self.data[0]
    }

    fn empty(&self) -> bool {
        self.data.is_empty()
    }
}
