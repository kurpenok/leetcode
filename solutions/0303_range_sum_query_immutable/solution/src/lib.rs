struct NumArray {
    nums: Vec<i32>,
}

impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        Self { nums }
    }

    fn sum_range(&self, left: i32, right: i32) -> i32 {
        let left = left as usize;
        let right = right as usize;

        let mut s = 0;
        for i in left..right + 1 {
            s += self.nums[i];
        }

        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let num_array = NumArray::new(vec![-2, 0, 3, -5, 2, -1]);

        assert_eq!(num_array.sum_range(0, 2), 1);
        assert_eq!(num_array.sum_range(2, 5), -1);
        assert_eq!(num_array.sum_range(0, 5), -3);
    }
}
