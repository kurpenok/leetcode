pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut intervals = intervals;
    intervals.sort_by(|a, b| a[0].cmp(&b[0]));

    let mut merged_intervals: Vec<Vec<i32>> = Vec::new();

    let mut start: i32 = intervals[0][0];
    let mut end: i32 = intervals[0][1];

    for interval in &intervals {
        if interval[0] <= end {
            end = std::cmp::max(end, interval[1]);
        } else if interval[0] > end {
            merged_intervals.push(vec![start, end]);
            start = interval[0];
            end = interval[1];
        }
    }
    merged_intervals.push(vec![start, end]);

    merged_intervals
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
            [[1, 6], [8, 10], [15, 18]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(merge(vec![vec![1, 4], vec![4, 5]]), [[1, 5]]);
    }
}
