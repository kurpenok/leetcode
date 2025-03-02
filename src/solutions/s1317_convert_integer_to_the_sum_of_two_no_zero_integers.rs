fn is_no_zero(a: i32) -> bool {
    !a.to_string().contains("0")
}

pub fn get_no_zero_integers(n: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    'main: for a in 1..=n {
        if is_no_zero(a) {
            for b in 1..=n - a {
                if a + b == n && is_no_zero(b) {
                    result.push(a);
                    result.push(b);
                    break 'main;
                }
            }
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(get_no_zero_integers(2), [1, 1]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(get_no_zero_integers(11), [2, 9]);
    }
}
