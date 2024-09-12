pub struct MyHashSet {
    data: Vec<i32>,
}

impl MyHashSet {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn add(&mut self, key: i32) {
        for element in &self.data {
            if *element == key {
                return;
            }
        }

        self.data.push(key);
    }

    pub fn remove(&mut self, key: i32) {
        let mut index: i32 = -1;

        for (i, element) in self.data.iter().enumerate() {
            if *element == key {
                index = i as i32;
                break;
            }
        }

        if index != -1 {
            self.data.remove(index as usize);
        }
    }

    pub fn contains(&self, key: i32) -> bool {
        for element in &self.data {
            if *element == key {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        let mut my_hashset = MyHashSet::new();

        my_hashset.add(1);
        assert_eq!(my_hashset.data, vec![1]);

        my_hashset.add(2);
        assert_eq!(my_hashset.data, vec![1, 2]);

        assert_eq!(my_hashset.contains(1), true);
        assert_eq!(my_hashset.contains(3), false);

        my_hashset.add(2);
        assert_eq!(my_hashset.data, vec![1, 2]);

        assert_eq!(my_hashset.contains(2), true);

        my_hashset.remove(2);
        assert_eq!(my_hashset.data, vec![1]);

        assert_eq!(my_hashset.contains(2), false);
    }
}
