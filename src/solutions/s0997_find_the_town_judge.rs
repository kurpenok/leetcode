pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
    if trust.len() == 0 && n == 1 {
        return 1;
    } else if trust.len() == 0 {
        return -1;
    }

    'main: for judge in 1..=n {
        let mut trust_count = 0;
        for i in 0..trust.len() {
            if judge == trust[i][0] {
                continue 'main;
            } else if trust[i][1] == judge {
                trust_count += 1;
            }
        }
        if trust_count == n - 1 {
            return judge;
        }
    }

    -1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(find_judge(2, vec![vec![1, 2]]), 2);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3]]), 3);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(find_judge(3, vec![vec![1, 3], vec![2, 3], vec![3, 1]]), -1);
    }
}
