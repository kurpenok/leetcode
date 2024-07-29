pub fn max_count(m: i32, n: i32, ops: Vec<Vec<i32>>) -> i32 {
    if ops.len() == 0 {
        return m * n;
    }

    let mut x: i32 = i32::MAX;
    let mut y: i32 = i32::MAX;

    for op in ops {
        if op[0] < x {
            x = op[0];
        }

        if op[1] < y {
            y = op[1];
        }
    }

    x * y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(max_count(3, 3, vec![vec![2, 2], vec![3, 3]]), 4);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(
            max_count(
                3,
                3,
                vec![
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3],
                    vec![2, 2],
                    vec![3, 3],
                    vec![3, 3],
                    vec![3, 3]
                ]
            ),
            4
        );
    }

    #[test]
    fn test_case_3() {
        assert_eq!(max_count(3, 3, vec![]), 9);
    }
}
