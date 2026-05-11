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
