pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = vec![0; nums1.len()];

    for i in 0..nums1.len() {
        let index = nums2.iter().position(|&n| n == nums1[i]).unwrap();

        for j in index + 1..nums2.len() {
            if nums2[j] > nums1[i] {
                result[i] = nums2[j];
                break;
            }
        }

        if result[i] == 0 {
            result[i] = -1;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
            vec![-1, 3, -1]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
            vec![3, -1]
        );
    }
}
