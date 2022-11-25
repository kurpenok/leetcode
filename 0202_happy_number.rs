use std::collections::HashSet;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        fn number_to_digits(n: i32) -> Vec<i32> {
            let mut digits: Vec<i32> = Vec::new();
            let mut number: i32 = n;
            while number >= 10 {
                digits.push(number % 10);
                number /= 10;
            }
            digits.push(number % 10);

            digits
        }

        let mut digits: Vec<i32> = number_to_digits(n);
        let mut sums: HashSet<i32> = HashSet::new();
        let mut sum_of_digits_of_number: i32 = 0;

        while sum_of_digits_of_number != 1 {
            sum_of_digits_of_number = 0;
            for digit in digits {
                sum_of_digits_of_number += digit.pow(2);
            }
            digits = number_to_digits(sum_of_digits_of_number);
            if !sums.insert(sum_of_digits_of_number) {
                return false;
            }
        }

        true
    }
}
