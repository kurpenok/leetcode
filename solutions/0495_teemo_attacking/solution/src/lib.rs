pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
    let mut time_of_poisoning: i32 = 0;

    for i in 0..time_series.len() - 1 {
        if time_series[i + 1] - time_series[i] < duration {
            time_of_poisoning += time_series[i + 1] - time_series[i];
        } else {
            time_of_poisoning += duration;
        }
    }

    time_of_poisoning + duration
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_poisoned_duration(vec![1, 4], 2), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_poisoned_duration(vec![1, 2], 2), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_poisoned_duration(vec![1, 2, 3, 4, 5], 5), 9);
    }
}
