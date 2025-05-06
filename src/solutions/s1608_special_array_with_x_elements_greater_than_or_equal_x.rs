pub fn special_array(nums: Vec<i32>) -> i32 {
    let arr_count = nums.iter().fold([0; 1001], |mut acc, &x| {
        acc[x as usize] += 1;
        acc
    });
    let mut prev_sum = 0;
    let l = nums.len() as i32;

    for i in 0..=l {
        let after_sum = l - prev_sum;
        prev_sum += arr_count[i as usize];

        if after_sum == i {
            return i;
        } else if prev_sum == l {
            break;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(special_array(vec![3, 5]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(special_array(vec![0, 0]), -1);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(special_array(vec![0, 4, 3, 0, 4]), 3);
    }
}
