pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();

    for num in left..=right {
        let mut flag: bool = false;

        for digit in num.to_string().chars() {
            if digit == '0' || num % digit.to_digit(10).unwrap() as i32 != 0 {
                flag = true;
                break;
            }
        }

        if !flag {
            result.push(num);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        assert_eq!(
            self_dividing_numbers(1, 22),
            vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
        );
    }

    #[test]
    fn test_case_2() {
        assert_eq!(self_dividing_numbers(47, 85), vec![48, 55, 66, 77]);
    }
}
