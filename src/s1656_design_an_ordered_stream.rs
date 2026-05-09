pub struct OrderedStream {
    values: Vec<Option<String>>,
    ptr: usize,
    n: usize,
}

impl OrderedStream {
    pub fn new(n: i32) -> Self {
        OrderedStream {
            values: vec![None; n as usize + 1],
            ptr: 1,
            n: n as usize,
        }
    }

    pub fn insert(&mut self, id_key: i32, value: String) -> Vec<String> {
        self.values[id_key as usize] = Some(value);

        let mut result = Vec::new();
        while self.ptr <= self.n && self.values[self.ptr].is_some() {
            if let Some(v) = self.values[self.ptr].take() {
                result.push(v);
                self.ptr += 1;
            }
        }

        result
    }
}
