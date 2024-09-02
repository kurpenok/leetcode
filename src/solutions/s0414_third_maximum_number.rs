pub fn third_max(nums: Vec<i32>) -> i32 {
    if nums.len() > 0 && nums.len() < 3 {
        return *nums.iter().max().unwrap();
    }

    let mut three: Vec<i32> = vec![nums[0], nums[1], nums[2]];
    three.sort();

    let mut first_max: i32 = three[2];
    let mut second_max: i32 = three[1];
    let mut third_max: i32 = three[0];

    let mut flag_all_equal: bool = first_max == second_max && second_max == third_max;
    let mut flag_f_s_equal: bool = first_max == second_max;
    let mut flag_s_t_equal: bool = second_max == third_max;

    for i in 3..nums.len() {
        println!("{} {} {}", first_max, second_max, third_max);

        if flag_all_equal && first_max < nums[i] {
            first_max = nums[i];
            flag_all_equal = false;
            flag_f_s_equal = false;
            continue;
        } else if flag_all_equal && third_max > nums[i] {
            third_max = nums[i];
            flag_all_equal = false;
            flag_s_t_equal = false;
            continue;
        }

        if flag_f_s_equal && first_max < nums[i] {
            first_max = nums[i];
            flag_f_s_equal = false;
            continue;
        } else if flag_f_s_equal && first_max > nums[i] && third_max < nums[i] {
            second_max = nums[i];
            flag_f_s_equal = false;
            continue;
        } else if flag_f_s_equal && third_max > nums[i] {
            second_max = third_max;
            third_max = nums[i];
            flag_f_s_equal = false;
            continue;
        }

        if flag_s_t_equal && first_max < nums[i] {
            second_max = first_max;
            first_max = nums[i];
            flag_s_t_equal = false;
            continue;
        } else if flag_s_t_equal && first_max > nums[i] && third_max < nums[i] {
            second_max = nums[i];
            flag_s_t_equal = false;
            continue;
        } else if flag_s_t_equal && third_max > nums[i] {
            third_max = nums[i];
            flag_s_t_equal = false;
            continue;
        }

        if first_max < nums[i] {
            third_max = second_max;
            second_max = first_max;
            first_max = nums[i];
        } else if second_max < nums[i] && first_max > nums[i] {
            third_max = second_max;
            second_max = nums[i];
        } else if third_max < nums[i] && second_max > nums[i] {
            third_max = nums[i];
        }
    }

    if flag_f_s_equal || flag_s_t_equal {
        return first_max;
    }

    third_max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(third_max(vec![3, 2, 1]), 1);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(third_max(vec![1, 2]), 2);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(third_max(vec![2, 2, 3, 1]), 1);
    }

    #[test]
    fn test_case_4() {
        assert_eq!(third_max(vec![3, 3, 3, 3, 4, 3, 2, 3, 3]), 2);
    }

    #[test]
    fn test_case_5() {
        assert_eq!(third_max(vec![1, 2, 2, 5, 3, 5]), 2);
    }

    #[test]
    fn test_case_6() {
        assert_eq!(third_max(vec![5, 2, 4, 1, 3, 6, 0]), 4);
    }
}
