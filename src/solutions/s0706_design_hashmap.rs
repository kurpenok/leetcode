pub struct MyHashMap {
    data: Vec<Vec<i32>>,
}

impl MyHashMap {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn put(&mut self, key: i32, value: i32) {
        for element in &mut self.data {
            if element[0] == key {
                element[1] = value;
                return;
            }
        }

        self.data.push(vec![key, value]);
    }

    pub fn get(&self, key: i32) -> i32 {
        for element in &self.data {
            if element[0] == key {
                return element[1];
            }
        }

        -1
    }

    pub fn remove(&mut self, key: i32) {
        let mut index: i32 = -1;

        for (i, element) in self.data.iter().enumerate() {
            if element[0] == key {
                index = i as i32;
                break;
            }
        }

        if index != -1 {
            self.data.remove(index as usize);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut my_hashmap = MyHashMap::new();

        my_hashmap.put(1, 1);
        assert_eq!(my_hashmap.data, vec![vec![1, 1]]);

        my_hashmap.put(2, 2);
        assert_eq!(my_hashmap.data, vec![vec![1, 1], vec![2, 2]]);

        assert_eq!(my_hashmap.get(1), 1);
        assert_eq!(my_hashmap.get(3), -1);

        my_hashmap.put(2, 1);
        assert_eq!(my_hashmap.data, vec![vec![1, 1], vec![2, 1]]);

        assert_eq!(my_hashmap.get(2), 1);

        my_hashmap.remove(2);
        assert_eq!(my_hashmap.data, vec![vec![1, 1]]);

        assert_eq!(my_hashmap.get(2), -1);
    }
}
