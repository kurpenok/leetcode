use std::collections::{HashMap, VecDeque};

pub struct LRUCache {
    access_counter: usize,
    cache_capacity: usize,
    cache: HashMap<i32, (i32, usize)>,
    queue: VecDeque<(i32, usize)>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        LRUCache {
            access_counter: 0,
            cache_capacity: capacity as usize,
            cache: HashMap::with_capacity(capacity as usize),
            queue: VecDeque::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        match self.cache.get(&key) {
            Some(&(value, _)) => {
                self.update_cache(key, value);
                value
            }
            None => -1,
        }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if !self.cache.contains_key(&key) && self.cache.len() == self.cache_capacity {
            let (key, _) = self.queue.pop_front().unwrap();
            self.cache.remove(&key);
        }

        self.update_cache(key, value);
    }

    pub fn update_cache(&mut self, key: i32, value: i32) {
        self.access_counter = self.access_counter.wrapping_add(1);
        self.cache.insert(key, (value, self.access_counter));
        self.queue.push_back((key, self.access_counter));

        loop {
            if let Some(&(key, access_counter_1)) = self.queue.front() {
                if let Some(&(_, access_counter_2)) = self.cache.get(&key) {
                    if access_counter_1 == access_counter_2 {
                        break;
                    }
                }
            }

            self.queue.pop_front();
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut lru_cache = LRUCache::new(2);

        lru_cache.put(1, 1);
        lru_cache.put(2, 2);

        assert_eq!(lru_cache.get(1), 1);

        lru_cache.put(3, 3);

        assert_eq!(lru_cache.get(2), -1);

        lru_cache.put(4, 4);

        assert_eq!(lru_cache.get(1), -1);
        assert_eq!(lru_cache.get(3), 3);
        assert_eq!(lru_cache.get(4), 4);
    }
}
