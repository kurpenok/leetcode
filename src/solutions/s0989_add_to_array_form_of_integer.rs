pub fn add_to_array_form(num: Vec<i32>, k: i32) -> Vec<i32> {
    let k_form: Vec<i32> = k
        .to_string()
        .chars()
        .into_iter()
        .map(|digit| digit.to_digit(10).unwrap() as i32)
        .collect();

    let mut new_form: Vec<i32> = Vec::new();
    let mut carry: i32 = 0;
    for i in 0..std::cmp::max(num.len(), k_form.len()) {
        let digit_1 = if i < num.len() {
            num[num.len() - i - 1]
        } else {
            0
        };
        let digit_2 = if i < k_form.len() {
            k_form[k_form.len() - i - 1]
        } else {
            0
        };
        let sum = digit_1 + digit_2 + carry;
        carry = sum / 10;
        new_form.push(sum % 10);
    }

    if carry > 0 {
        new_form.push(carry);
    }
    new_form.reverse();

    new_form
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(add_to_array_form(vec![1, 2, 0, 0], 34), [1, 2, 3, 4]);
    }

    #[test]
    fn test_case_2() {
        assert_eq!(add_to_array_form(vec![2, 7, 4], 181), [4, 5, 5]);
    }

    #[test]
    fn test_case_3() {
        assert_eq!(add_to_array_form(vec![2, 1, 5], 806), [1, 0, 2, 1]);
    }
}
