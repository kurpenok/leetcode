pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    let mut united: Vec<i32> = Vec::new();
    let united_len = nums1.len() + nums2.len();

    let mut i: usize = 0;
    let mut j: usize = 0;

    while i != nums1.len() && j != nums2.len() {
        if nums1[i] < nums2[j] {
            united.push(nums1[i]);
            i += 1;
        } else {
            united.push(nums2[j]);
            j += 1;
        }
    }

    while i != nums1.len() {
        united.push(nums1[i]);
        i += 1;
    }
    while j != nums2.len() {
        united.push(nums2[j]);
        j += 1;
    }

    if united_len % 2 == 0 {
        let a = united[united.len() / 2 - 1];
        let b = united[united.len() / 2];
        return (a + b) as f64 / 2.0;
    } else {
        return united[united.len() / 2] as f64;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums1 = vec![1, 3];
        let nums2 = vec![2];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.0);
    }

    #[test]
    fn test_case_2() {
        let nums1 = vec![1, 2];
        let nums2 = vec![3, 4];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 2.5);
    }

    #[test]
    fn test_case_3() {
        let nums1 = vec![];
        let nums2 = vec![1];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), 1.0);
    }

    #[test]
    fn test_case_4() {
        let nums1 = vec![3];
        let nums2 = vec![-2, -1];
        assert_eq!(find_median_sorted_arrays(nums1, nums2), -1.0);
    }
}
