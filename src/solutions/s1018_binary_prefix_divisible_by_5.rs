pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
    let mut divides = Vec::with_capacity(nums.len());
    let mut val = 0;

    for num in &nums {
        val = (val << 1 | num) % 5;
        divides.push(val == 0);
    }

    divides
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(prefixes_div_by5(vec![0, 1, 1]), [true, false, false]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(prefixes_div_by5(vec![1, 1, 1]), [false, false, false]);
    }
}
