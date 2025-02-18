pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let mut new_intervals: Vec<Vec<i32>> = Vec::new();
    let mut new_interval = new_interval;

    for i in 0..intervals.len() {
        if intervals[i][1] < new_interval[0] {
            new_intervals.push(intervals[i].clone());
        } else if intervals[i][0] > new_interval[1] {
            new_intervals.push(new_interval.clone());
            new_interval = intervals[i].clone();
        } else if intervals[i][1] >= new_interval[0] || intervals[i][0] <= new_interval[1] {
            new_interval[0] = std::cmp::min(new_interval[0], intervals[i][0]);
            new_interval[1] = std::cmp::max(new_interval[1], intervals[i][1]);
        }
    }

    new_intervals.push(new_interval.clone());

    new_intervals
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
            [[1, 5], [6, 9]]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            insert(
                vec![
                    vec![1, 2],
                    vec![3, 5],
                    vec![6, 7],
                    vec![8, 10],
                    vec![12, 16]
                ],
                vec![4, 8]
            ),
            [[1, 2], [3, 10], [12, 16]]
        );
    }
}
