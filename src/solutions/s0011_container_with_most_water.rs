fn get_value(height: &Vec<i32>, i: usize, j: usize) -> i32 {
    let distance = (i as i32 - j as i32).abs();
    let min_max_height = std::cmp::min(height[i], height[j]);

    distance * min_max_height
}

pub fn max_area(height: Vec<i32>) -> i32 {
    let mut left = 0;
    let mut right = height.len() - 1;

    let mut max_value = get_value(&height, left, right);

    while left < right {
        max_value = std::cmp::max(max_value, get_value(&height, left, right));

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }

    max_value
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(max_area(vec![1, 1]), 1);
    }
}
