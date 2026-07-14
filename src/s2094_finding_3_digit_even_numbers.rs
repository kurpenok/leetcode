use std::collections::BTreeSet;

pub fn find_even_numbers(digits: Vec<i32>) -> Vec<i32> {
    let mut combinations = BTreeSet::new();
    for i in 0..digits.len() {
        if digits[i] == 0 {
            continue;
        }
        for j in 0..digits.len() {
            if i == j {
                continue;
            }
            for k in 0..digits.len() {
                if i != k && j != k && digits[k] % 2 == 0 {
                    combinations.insert(digits[i] * 100 + digits[j] * 10 + digits[k]);
                }
            }
        }
    }

    combinations.into_iter().collect()
}
