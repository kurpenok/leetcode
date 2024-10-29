pub fn di_string_match(s: String) -> Vec<i32> {
    let mut perm: Vec<i32> = Vec::with_capacity(s.len() + 1);

    let mut min_val: i32 = 0;
    let mut max_val: i32 = s.len() as i32;

    s.chars().for_each(|c| match c {
        'I' => {
            perm.push(min_val);
            min_val += 1
        }
        'D' => {
            perm.push(max_val);
            max_val -= 1;
        }
        _ => {
            return;
        }
    });
    perm.push(max_val);

    perm
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(di_string_match("IDID".to_string()), vec![0, 4, 1, 3, 2]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(di_string_match("III".to_string()), vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(di_string_match("DDI".to_string()), vec![3, 2, 0, 1]);
    }
}
