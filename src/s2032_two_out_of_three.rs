use std::collections::HashSet;

pub fn two_out_of_three(nums1: Vec<i32>, nums2: Vec<i32>, nums3: Vec<i32>) -> Vec<i32> {
    let set_1: HashSet<i32> = nums1.into_iter().collect();
    let set_2: HashSet<i32> = nums2.into_iter().collect();
    let set_3: HashSet<i32> = nums3.into_iter().collect();

    let intersection_1_2 = &set_1 & &set_2;
    let intersection_1_3 = &set_1 & &set_3;
    let intersection_2_3 = &set_2 & &set_3;

    let mut intersection: Vec<i32> = intersection_1_2
        .into_iter()
        .chain(intersection_1_3.into_iter())
        .chain(intersection_2_3.into_iter())
        .collect::<HashSet<i32>>()
        .into_iter()
        .collect();
    intersection.sort();

    intersection
}
